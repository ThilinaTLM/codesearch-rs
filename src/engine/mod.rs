use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use tantivy::{self, doc};

pub use search_engine::FileSearchEngine;
pub use simple_schema::SchemaWrapperModel;

mod search_engine;
mod simple_schema;
mod search_error;
mod metadata;

pub struct SearchOptions {
    pub query: String,
    pub limit: usize,
}

pub struct RepoInfo {
    pub name: String,
    pub path: String,
    pub type_: String,
    pub last_indexed_time: Option<u64>,
    pub number_of_indexed_files: Option<u64>,
    pub indexing_status: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResultItem {
    pub _score: f32,
    #[serde(flatten)]
    pub data: SchemaWrapperModel,
}

#[async_trait]
pub trait SearchEngine {
    async fn search(&self, options: SearchOptions) -> Result<Vec<ResultItem>>;
    async fn get_repo_list(&self) -> Result<Vec<RepoInfo>>;
}