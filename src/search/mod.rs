use serde::Serialize;
use tantivy::{self, doc};

use crate::search::simple_text::SimpleTextDto;
pub use crate::search::engine::FileSearchEngine;

mod engine;
mod simple_text;
mod error;


pub struct SearchOptions {
    pub query: String,
}

#[derive(Debug, Serialize)]
pub struct ResultItem {
    pub data: SimpleTextDto,
    pub score: f32,
}

#[derive(Debug, Serialize)]
pub struct SearchResult {
    pub results: Vec<ResultItem>,
}

