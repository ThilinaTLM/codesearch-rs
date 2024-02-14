use std::fs;
use std::path::Path;
use std::sync::{Arc, RwLock};

use anyhow::Result;
use async_trait::async_trait;
use log;
use rayon::prelude::*;
use tantivy::directory::MmapDirectory;
use walkdir::{DirEntry, WalkDir};

use crate::config;
use crate::config::Config;
use crate::engine::{simple_schema, ResultItem, SearchEngine, SearchOptions};
use crate::engine::metadata::EngineMetadataRepo;
use crate::engine::simple_schema::SimpleSchemaWrapper;

pub struct FileSearchEngine {
    config: Config,
    schema_wrapper: SimpleSchemaWrapper,
    index: tantivy::Index,
    meta_data_repo: EngineMetadataRepo,
}

impl FileSearchEngine {


    pub fn new(config: &Config) -> Result<Self> {

        // load meta data db
        log::info!("Opening meta data db");
        let meta_data = EngineMetadataRepo::new(Path::new(&config.index.metadata_dir()))?;
        log::info!("Meta data db opened successfully");

        log::info!("Opening index");
        let schema_wrapper = SimpleSchemaWrapper::create().unwrap();
        let index_path = MmapDirectory::open(Path::new(&config.index.index_dir()))
            .map_err(anyhow::Error::new)?;
        let index = tantivy::Index::open_or_create(index_path, schema_wrapper.get_schema().clone())
            .map_err(anyhow::Error::new)?;
        log::info!("Index opened successfully");

        // starting indexing repos that are not indexed yet
        for repo in &config.repos {
            if !meta_data.is_repo_indexed(&repo.name) {
                log::info!("Indexing repo: {}", repo.name);
                let engine = FileSearchEngine {
                    config: config.clone(),
                    schema_wrapper: schema_wrapper.clone(),
                    index: index.clone(),
                    meta_data_repo: meta_data.clone(),
                };
                engine.index_repo(repo)?;
                meta_data.mark_repo_indexed(&repo.name)?;
            }
        }

        Ok(Self {
            config: config.clone(),
            schema_wrapper,
            index,
            meta_data_repo: meta_data,
        })
    }

    fn filter_skip_patterns(&self, entry: &DirEntry, repo: &config::Repo) -> bool {
        let path = entry.path();
        let path_str = path.to_str().unwrap();
        repo.skip_dir_patterns.iter().any(|pattern| {
            path_str.contains(pattern)
        })
    }

    fn index_repo(&self, repo: &config::Repo) -> Result<(), SearchError> {
        let index = &self.index;
        let index_writer_arc = Arc::new(RwLock::new(index.writer(50_000_000)?));

        let walker = WalkDir::new(&repo.path).into_iter();
        walker.filter_entry(|e| !self.filter_skip_patterns(e, repo))
            .filter_map(|e| e.ok())
            .par_bridge()
            .for_each(|entry| {
                match entry.path().extension() {
                    Some(ext) => {
                        if !repo.include_file_extensions.contains(&ext.to_str().unwrap().to_string()) {
                            return;
                        }
                    }
                    None => {
                        return;
                    }
                }

                log::trace!("Indexing file: {:?}", entry.path());

                let repo_name = repo.name.clone();
                let repo_path = repo.path.clone();
                let repo_type = repo.type_.clone();
                let file_name = entry.file_name().to_str().unwrap().to_string();
                let file_path = entry.path().strip_prefix(&repo.path).unwrap().to_str().unwrap().to_string();
                let file_ext = match entry.path().extension() {
                    Some(ext) => ext.to_str().unwrap().to_string(),
                    None => "".to_string(),
                };
                let file_size = entry.metadata().unwrap().len();
                let file_last_updated = entry.metadata().unwrap().modified().unwrap();
                let file_content = fs::read_to_string(entry.path()).unwrap();

                let data = simple_schema::SimpleSchemaModel {
                    repo_name,
                    repo_path,
                    repo_type,
                    file_name,
                    file_path,
                    file_ext,
                    file_size,
                    file_content,
                    last_updated: file_last_updated.into(),
                };
                let doc = self.schema_wrapper.create_document(data);

                let index_writer = index_writer_arc.read().unwrap();
                index_writer.add_document(doc).unwrap();
            });

        let mut index_writer = index_writer_arc.write().unwrap();
        index_writer.commit()?;

        Ok(())
    }
}


#[async_trait]
impl SearchEngine for FileSearchEngine {
    async fn search(&self, options: SearchOptions) -> Result<Vec<ResultItem>, SearchError> {
        log::info!("Executing engine with query: {}", options.query);
        let index = &self.index;
        let index_reader = index.reader()?;
        let searcher = index_reader.searcher();

        let query = options.query;
        let limit = options.limit;
        let query_parser = tantivy::query::QueryParser::for_index(&index, vec![
            self.schema_wrapper.get_field(simple_schema::SimpleSchemaFields::FileContent),
            self.schema_wrapper.get_field(simple_schema::SimpleSchemaFields::FileName),
            self.schema_wrapper.get_field(simple_schema::SimpleSchemaFields::FilePath),
        ]);

        let query = query_parser.parse_query(&query)?;
        let top_docs = searcher.search(&query, &tantivy::collector::TopDocs::with_limit(limit))?;

        let mut results = Vec::new();

        for (score, doc_address) in top_docs {
            let retrieved_doc = searcher.doc(doc_address.clone()).unwrap();
            let code_file_dto = self.schema_wrapper.create_code_file_dto(&retrieved_doc).unwrap();
            results.push(ResultItem {
                data: code_file_dto,
                _score: score,
            });
        }

        Ok(results)
    }
}
