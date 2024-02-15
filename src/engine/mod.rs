use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use tantivy::{self, doc};
use anyhow::Result;

pub use simple_schema::SchemaWrapperModel;
pub use search_engine::FileSearchEngine;

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
    pub data: SchemaWrapperModel,
}

#[async_trait]
pub trait SearchEngine {
    async fn search(&self, options: SearchOptions) -> Result<Vec<ResultItem>>;
}