use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use tantivy::{self, doc};

pub use simple_schema::SimpleSchemaModel;
pub use search_engine::FileSearchEngine;
pub use search_error::SearchError;

mod search_engine;
mod simple_schema;
mod search_error;
mod metadata;

pub struct SearchOptions {
    pub query: String,
    pub limit: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResultItem  {
    pub _score: f32,
    #[serde(flatten)]
    pub data: SimpleSchemaModel,
}

#[async_trait]
pub trait SearchEngine {
    async fn search(&self, options: SearchOptions) -> Result<Vec<ResultItem>, SearchError>;
}