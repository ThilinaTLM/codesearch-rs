use std::fs;
use std::path::PathBuf;
use std::sync::Arc;

use walkdir::{DirEntry, WalkDir};

use crate::config::Config;
use crate::search::{SearchOptions, SearchResult};
use crate::search::error::SearchError;
use crate::search::simple_text::SimpleTextIndex;

pub struct FileSearchEngine {
    config: Config,
    index: Arc<SimpleTextIndex>,
}

impl FileSearchEngine {
    pub fn new(config: &Config) -> tantivy::Result<Self> {
        let index = if config.indexer.use_temporary_index {
            let index_path = tantivy::directory::MmapDirectory::create_from_tempdir()?;
            SimpleTextIndex::new(index_path)?
        } else {
            let index_path = PathBuf::from("index");
            if !index_path.exists() {
                fs::create_dir(&index_path)?;
            }
            let index_path = tantivy::directory::MmapDirectory::open(index_path)?;
            SimpleTextIndex::new(index_path)?
        };

        Ok(Self {
            index: Arc::new(index),
            config: config.clone(),
        })
    }

    fn should_skip(&self, entry: &DirEntry) -> bool {
        let path = entry.path();
        let path_str = path.to_str().unwrap();
        self.config.indexer.skip_patterns.iter().any(|pattern| path_str.contains(pattern))
    }

    pub(crate) async fn initialize(&self) -> Result<(), SearchError> {
        let config = self.config.clone();
        for repo in &config.repos {
            self.index_repo(repo)?;
        }
        Ok(())
    }

    fn index_repo(&self, repo: &crate::config::Repo) -> Result<(), SearchError> {
        let unknown_os_str = std::ffi::OsStr::new("unknown");
        let index = &self.index;

        // Walk through the repo and index the files
        let walker = WalkDir::new(&repo.path).into_iter();
        for entry in walker.filter_entry(|e| !self.should_skip(e)) {
            let entry = entry?;
            if entry.file_type().is_file() {

                // repo name, path, type
                let repo_name = repo.name.clone();
                let repo_path = repo.path.clone();
                let repo_type = repo.type_.clone();

                // file name, path, ext, size, last_updated, language, content
                let file_name = entry.file_name().to_str().unwrap().to_string();
                let file_path = entry.path().strip_prefix(&repo.path).unwrap().to_str().unwrap().to_string();
                let file_ext = entry.path().extension().or(Some(unknown_os_str)).unwrap()
                    .to_str().unwrap().to_string();
                let file_size = entry.metadata()?.len();
                let file_last_updated = entry.metadata()?.modified().unwrap();
                let file_language = mime_guess::from_path(&entry.path()).first_or_octet_stream().to_string();
                let file_content = fs::read_to_string(entry.path())?;

                let data = crate::search::simple_text::SimpleTextDto {
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
                index.add_record(data);
            }
        }
        Ok(())
    }

    async fn search(&self, options: SearchOptions) -> Result<SearchResult, SearchError> {
        let index = self.index.clone();
        let query = options.query;
        let results = index.simple_search(&query)?;
        Ok(SearchResult {
            results
        })
    }
}
