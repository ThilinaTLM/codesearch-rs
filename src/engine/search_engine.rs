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
    schema_wrapper: Arc<SimpleSchemaWrapper>,
    index: Arc<tantivy::Index>,
    meta_data_repo: Arc<EngineMetadataRepo>,
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
            schema_wrapper: Arc::new(schema_wrapper),
            index: Arc::new(index),
            meta_data_repo: Arc::new(meta_data),
        })
    }

    pub async fn index_repo(&self, repo_name: String) -> Result<()> {

        let index = self.index.clone();
        let repo = self.config.repos.iter()
            .find(|r| r.name == repo_name).ok_or_else(|| anyhow::anyhow!("Repo not found"))
            .map(|r| r.clone())?;
        let meta_data_repo = self.meta_data_repo.clone();
        let schema_wrapper = self.schema_wrapper.clone();

        let filter_skip_patterns = move | entry: &DirEntry | {
            let path = entry.path();
            let path_str = path.to_str().unwrap();
            repo.skip_dir_patterns.iter().any(|pattern| {
                path_str.contains(pattern)
            })
        };

        let handle: tokio::task::JoinHandle<Result<()>> = tokio::spawn(async move {
            log::info!("Indexing repo: {}", repo.name);
            let index_writer = index.writer(50_000_000).map_err(anyhow::Error::new)?;
            let index_writer_arc = Arc::new(RwLock::new(index_writer));
            let last_indexed_timestamp = meta_data_repo.get_last_indexed_timestamp(&repo.name)
                .unwrap_or(Some(0)).ok_or_else(|| anyhow::anyhow!("Failed to get last indexed timestamp"))?;

            let walker = WalkDir::new(&repo.path).into_iter();
            walker.filter_entry(|e| filter_skip_patterns(e))
                .filter_map(|e| e.ok())
                .par_bridge()
                .for_each(|entry| {
                    log::trace!("Checking that file extension in the allowed list: {:?}", entry.path().extension());
                    if let Some(ext) = entry.path().extension().and_then(std::ffi::OsStr::to_str) {
                        if !repo.include_file_extensions.contains(&ext.to_string()) {
                            return;
                        }
                    } else {
                        return;
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
                    let doc = schema_wrapper.create_document(data);

                    let index_writer = index_writer_arc.read().unwrap();
                    index_writer.add_document(doc).unwrap();
                });

            let mut index_writer = index_writer_arc.write().unwrap();
            index_writer.commit().map_err(anyhow::Error::new)?;
            meta_data_repo.mark_repo_indexing_completed(&repo.name)?;
            Ok(())
        });

        handle.await??;

        Ok(())
    }

    pub fn get_repo_indexing_status(&self, repo_name: String) -> Result<String> {
        self.meta_data_repo.get_indexing_status(&repo_name)
    }

    pub fn get_list_of_repos(&self) -> Vec<String> {
        self.config.repos.iter().map(|r| r.name.clone()).collect()
    }

}


#[async_trait]
impl SearchEngine for FileSearchEngine {

    pub async fn search(&self, options: SearchOptions) -> Result<Vec<ResultItem>> {
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
