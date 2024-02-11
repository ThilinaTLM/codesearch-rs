use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use tantivy::{self, doc};

pub use code_schema::CodeFileDto;
pub use fs_search_engine::FileSearchEngine;
pub use search_error::SearchError;

mod fs_search_engine;
mod code_schema;
mod search_error;

pub struct SearchOptions {
    pub query: String,
    pub limit: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResultItem {
    pub data: CodeFileDto,
    pub score: f32,
}

#[async_trait]
pub trait SearchEngine {
    async fn search(&self, options: SearchOptions) -> Result<Vec<ResultItem>, SearchError>;
}