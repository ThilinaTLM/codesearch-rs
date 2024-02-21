
pub use search_engine::FileSearchEngine;
pub use simple_schema::SchemaWrapperModel;

mod search_engine;
mod simple_schema;
mod search_error;
mod metadata;

pub struct SearchQueryOptions {
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

pub struct SearchResultItem {
    pub _score: f32,
    pub data: SchemaWrapperModel,
}