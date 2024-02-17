use std::fs;
use std::path::Path;
use std::sync::{Arc, RwLock};

use anyhow::Result;
use async_trait::async_trait;
use log;
use rayon::prelude::*;
use tantivy::directory::MmapDirectory;
use tantivy::schema::IndexRecordOption;
use walkdir::{DirEntry, WalkDir};

use crate::config::Config;
use crate::engine::{RepoInfo, ResultItem, SearchEngine, SearchOptions, simple_schema};
use crate::engine::metadata::EngineMetadataRepo;
use crate::engine::simple_schema::SimpleSchemaWrapper;

pub struct FileSearchEngine {
    config: Arc<Config>,
    schema_wrapper: Arc<SimpleSchemaWrapper>,
    index: Arc<tantivy::Index>,
    meta_data_repo: Arc<EngineMetadataRepo>,
}

impl FileSearchEngine {
    pub fn new(config: Arc<Config>) -> Result<Self> {
        // load meta data db
        log::info!("Opening meta data db");
        let meta_data = EngineMetadataRepo::new(Path::new(&config.index.metadata_dir()))?;
        log::info!("Meta data db opened successfully");

        log::info!("Opening index");
        let schema_wrapper = SimpleSchemaWrapper::create().unwrap();
        let index_dir = config.index.index_dir();
        if Path::new(&index_dir).exists() {
            log::info!("Index directory already exists at: {}", index_dir);
        } else {
            log::info!("Index directory does not exist, creating at: {}", index_dir);
            fs::create_dir_all(&index_dir)
                .map_err(|e| anyhow::anyhow!("Failed to create index directory: {}", e))?;
        }
        let index_path = MmapDirectory::open(index_dir)
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

        log::info!("Indexing repo: {}", repo.name);
        let index_writer = index.writer(50_000_000).map_err(anyhow::Error::new)?;
        let index_writer_arc = Arc::new(RwLock::new(index_writer));
        let last_indexed_timestamp = meta_data_repo.get_last_indexed_timestamp(&repo.name).unwrap().unwrap_or_else(|| 0);

        let filter_ignore_directories = move |entry: &DirEntry| {
            let path = entry.path();
            let path_str = path.to_str().unwrap();
            !repo.skip_dir_patterns.iter().any(|pattern| {
                path_str.contains(pattern)
            })
        };

        let filter_file_extensions = move |entry: &DirEntry| {
            let path = entry.path();
            if let Some(ext) = path.extension().and_then(std::ffi::OsStr::to_str) {
                match repo.include_file_extensions.contains(&ext.to_string()) {
                    true => true,
                    false => {
                        log::trace!("File extension not in the allowed list, skipping");
                        false
                    }
                }
            } else {
                log::trace!("File has no extension, skipping");
                false
            }
        };

        let walker = WalkDir::new(&repo.path).into_iter();
        let update_count = walker
            .filter_entry(filter_ignore_directories)
            .filter_map(|e| e.ok())
            .par_bridge()
            .filter(filter_file_extensions)
            .map(|entry| {
                log::trace!("Checking that file has been modified since last indexing");
                if let Ok(metadata) = entry.metadata() {
                    if metadata.modified().unwrap().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() < last_indexed_timestamp {
                        return 0;
                    }
                } else {
                    log::error!("Failed to read file metadata: {}", entry.path().display());
                    return 0;
                }
                log::trace!("Indexing file: {:?}", entry.path());

                // reading data
                let file_content = match fs::read_to_string(entry.path()) {
                    Ok(content) => content,
                    Err(e) => {
                        log::error!("Failed to read file content: {}, error: {}", entry.path().display(), e);
                        return 0;
                    }
                };

                let data = simple_schema::SchemaWrapperModel {
                    repo_name: repo.name.clone(),
                    repo_path: repo.path.clone(),
                    repo_type: repo.type_.clone(),
                    file_name: entry.file_name().to_str().unwrap().to_string(),
                    file_path: entry.path().strip_prefix(&repo.path).unwrap().to_str().unwrap().to_string(),
                    file_ext: entry.path().extension().map_or_else(|| "".to_string(), |ext| ext.to_str().unwrap().to_string()),
                    file_size: entry.metadata().unwrap().len(),
                    file_content,
                    last_updated: entry.metadata().unwrap().modified().unwrap().into(),
                };
                let document = schema_wrapper.create_document(data);

                let index_writer = index_writer_arc.read().unwrap();
                index_writer.add_document(document).unwrap();
                return 1;
            }).reduce(|| 0, |a, b| a + b);

        let mut index_writer = index_writer_arc.write().unwrap();
        index_writer.commit().map_err(anyhow::Error::new)?;
        meta_data_repo.mark_repo_indexing_completed(&repo.name)?;
        log::info!("Indexed {} files", update_count);
        log::info!("Indexing completed successfully");

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

        let query = tantivy::query::TermQuery::new(
            tantivy::Term::from_field_text(self.schema_wrapper.get_field(simple_schema::SchemaWrapperFields::FileContent), &query),
            IndexRecordOption::WithFreqsAndPositions,
        );
        let top_docs = searcher.search(&query, &tantivy::collector::TopDocs::with_limit(limit))?;

        let mut results = Vec::new();
        for (score, doc_address) in top_docs {
            let retrieved_doc = searcher.doc(doc_address.clone()).unwrap();
            let code_file_dto = self.schema_wrapper.get_model_from_doc(&retrieved_doc).unwrap();
            results.push(ResultItem {
                data: code_file_dto,
                _score: score,
            });
        }
        Ok(results)
    }

    async fn get_repo_list(&self) -> Result<Vec<RepoInfo>> {
        let repos = self.config.repos.iter()
            .map(|r| {
                let last_indexed_time = self.meta_data_repo.get_last_indexed_timestamp(&r.name).unwrap_or(None);
                let number_of_indexed_files = self.meta_data_repo.get_number_of_indexed_files(&r.name).unwrap_or(None);
                let indexing_status = self.meta_data_repo.get_indexing_status(&r.name).unwrap_or("Not indexed".to_string());
                RepoInfo {
                    name: r.name.clone(),
                    path: r.path.clone(),
                    type_: r.type_.clone(),
                    last_indexed_time,
                    number_of_indexed_files,
                    indexing_status,
                }
            })
            .collect();
        Ok(repos)
    }
}
