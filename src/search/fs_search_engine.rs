use std::fs;
use std::path::PathBuf;
use std::sync::{Arc, RwLock};

use async_trait::async_trait;
use log;
use rayon::prelude::*;
use walkdir::{DirEntry, WalkDir};

use crate::config;
use crate::config::Config;
use crate::search::{code_schema, ResultItem, SearchEngine, SearchOptions};
use crate::search::code_schema::CodeFileSchema;
use crate::search::search_error::SearchError;

pub struct FileSearchEngine {
    config: Config,
    schema: CodeFileSchema,
    index: tantivy::Index,
}

impl FileSearchEngine {
    pub fn new(config: &Config) -> tantivy::Result<Self> {
        log::info!("Starting FS Search Engine");
        let index_path = if config.indexer.use_temporary_index {
            let index_path = tantivy::directory::MmapDirectory::create_from_tempdir()?;
            log::info!("Using temporary index: {:?}", index_path);
            index_path
        } else {
            let index_path = config.indexer.clone().index_path.unwrap().to_string();
            let index_path = PathBuf::from(index_path);
            if !index_path.exists() {
                fs::create_dir(&index_path)?;
            }
            let index_path = tantivy::directory::MmapDirectory::open(index_path)?;
            log::info!("Using index: {:?}", index_path);
            index_path
        };

        log::info!("Opening index");
        let code_file_schema = CodeFileSchema::create().unwrap();
        let index = tantivy::Index::open_or_create(index_path, code_file_schema.get_schema().clone())?;
        log::info!("Index opened successfully");

        Ok(Self {
            index,
            schema: code_file_schema,
            config: config.clone(),
        })
    }

    fn filter_skip_patterns(&self, entry: &DirEntry, repo: &config::Repo) -> bool {
        let path = entry.path();
        let path_str = path.to_str().unwrap();
        repo.skip_patterns.iter().any(|pattern| {
            path_str.contains(pattern)
        })
    }

    pub(crate) async fn initialize(&self) -> Result<(), SearchError> {
        log::info!("Initializing index for FileSearchEngine");

        if self.config.indexer.force_reindex {
            let config = self.config.clone();
            for repo in &config.repos {
                log::info!("Start indexing repo: {}", repo.name);
                self.adding_repo_files_to_index(repo)?;
                log::info!("Finished indexing repo: {}", repo.name);
            }
        } else {
            log::info!("Skipping indexing because force_reindex is false");
        }

        Ok(())
    }

    fn adding_repo_files_to_index(&self, repo: &config::Repo) -> Result<(), SearchError> {
        let index = &self.index;
        let index_writer_arc = Arc::new(RwLock::new(index.writer(50_000_000)?));

        let walker = WalkDir::new(&repo.path).into_iter();
        walker.filter_entry(|e| !self.filter_skip_patterns(e, repo))
            .filter_map(|e| e.ok())
            .par_bridge()
            .for_each(|entry| {
                match entry.path().extension() {
                    Some(ext) => {
                        if !repo.allowed_file_extensions.contains(&ext.to_str().unwrap().to_string()) {
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
                let file_language = mime_guess::from_path(&entry.path()).first_or_octet_stream().to_string();
                let file_content = fs::read_to_string(entry.path()).unwrap();

                let data = crate::search::code_schema::CodeFileDto {
                    repo_name,
                    repo_path,
                    repo_type,
                    file_name,
                    file_path,
                    file_ext,
                    file_size,
                    file_last_updated: file_last_updated.into(),
                    file_language,
                    file_content,
                };
                let doc = self.schema.create_document(data);

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
        log::info!("Executing search with query: {}", options.query);
        let index = &self.index;
        let index_reader = index.reader()?;
        let searcher = index_reader.searcher();

        let query = options.query;
        let limit = options.limit;
        let query_parser = tantivy::query::QueryParser::for_index(&index, vec![
            self.schema.get_field(code_schema::CodeSchemaFields::FileContent),
            self.schema.get_field(code_schema::CodeSchemaFields::FileName),
            self.schema.get_field(code_schema::CodeSchemaFields::FilePath),
        ]);

        let query = query_parser.parse_query(&query)?;
        let top_docs = searcher.search(&query, &tantivy::collector::TopDocs::with_limit(limit))?;

        let mut results = Vec::new();

        for (score, doc_address) in top_docs {
            let retrieved_doc = searcher.doc(doc_address.clone()).unwrap();
            let code_file_dto = self.schema.create_code_file_dto(&retrieved_doc).unwrap();
            results.push(ResultItem {
                data: code_file_dto,
                score,
            });
        }

        Ok(results)
    }
}
