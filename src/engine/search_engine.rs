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

    fn index_repo(&self, repo: &config::Repo) -> Result<()> {

        let index = &self.index;
        let index_writer_arc = Arc::new(RwLock::new(index.writer(50_000_000)?));
        let last_indexed_timestamp = self.meta_data_repo.get_last_indexed_timestamp(&repo.name)
            .unwrap_or(Some(0)).unwrap();

        let walker = WalkDir::new(&repo.path).into_iter();
        walker.filter_entry(|e| !self.filter_skip_patterns(e, repo))
            .filter_map(|e| e.ok())
            .par_bridge()
            .for_each(|entry| {

                log::trace!("Checking that file extension in the allowed list: {:?}", entry.path().extension());
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

                log::trace!("Checking that file has been modified since last indexing");
                if let Ok(metadata) = entry.metadata() {
                    if metadata.modified().unwrap().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() < last_indexed_timestamp {
                        return;
                    }
                }

                log::trace!("Indexing file: {:?}", entry.path());
                let data = simple_schema::SchemaWrapperModel {
                    repo_name: repo.name.clone(),
                    repo_path: repo.path.clone(),
                    repo_type: repo.type_.clone(),
                    file_name: entry.file_name().to_str().unwrap().to_string(),
                    file_path: entry.path().strip_prefix(&repo.path).unwrap().to_str().unwrap().to_string(),
                    file_ext: entry.path().extension().map_or_else(|| "".to_string(), |ext| ext.to_str().unwrap().to_string()),
                    file_size: entry.metadata().unwrap().len(),
                    file_content: fs::read_to_string(entry.path()).unwrap(),
                    last_updated: entry.metadata().unwrap().modified().unwrap().into(),
                };
                let doc = self.schema_wrapper.create_document(data);

                let index_writer = index_writer_arc.read().unwrap();
                index_writer.add_document(doc).unwrap();
            });

        let mut index_writer = index_writer_arc.write().unwrap();
        index_writer.commit()?;
        let timestamp = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();
        self.meta_data_repo.set_last_indexed_timestamp(&repo.name, timestamp)?;

        Ok(())
    }
}


#[async_trait]
impl SearchEngine for FileSearchEngine {
    async fn search(&self, options: SearchOptions) -> Result<Vec<ResultItem>> {
        log::info!("Executing engine with query: {}", options.query);
        let index = &self.index;
        let index_reader = index.reader()?;
        let searcher = index_reader.searcher();

        let query = options.query;
        let limit = options.limit;
        let query_parser = tantivy::query::QueryParser::for_index(&index, vec![
            self.schema_wrapper.get_field(simple_schema::SchemaWrapperFields::FileContent),
            self.schema_wrapper.get_field(simple_schema::SchemaWrapperFields::FileName),
            self.schema_wrapper.get_field(simple_schema::SchemaWrapperFields::FilePath),
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
