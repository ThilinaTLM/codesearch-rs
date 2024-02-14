use serde::{Deserialize, Serialize};

use crate::engine::ResultItem;

#[derive(Serialize, Deserialize)]
pub struct StandardResponse<T> where T: Serialize {
    pub(crate) data: Option<T>,
    pub(crate) error: Option<String>,
    pub(crate) time_taken: Option<u64>,
}


#[derive(Serialize, Deserialize)]
pub struct SearchRequest {
    pub query: String,
    pub limit: Option<usize>,
}

#[derive(Serialize, Deserialize)]
pub struct SearchResponse {
    pub results: Vec<ResultItem>,
    pub time_taken: u64,
}


#[derive(Serialize, Deserialize)]
pub struct HealthCheckResponse {
    pub status: String,
}