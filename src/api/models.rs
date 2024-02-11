use serde::{Deserialize, Serialize};

use crate::search::ResultItem;

#[derive(Serialize, Deserialize)]
pub struct StandardResponse<T> where T: Serialize {
    pub(crate) data: Option<T>,
    pub(crate) error: Option<String>,
}


#[derive(Serialize, Deserialize)]
pub struct SearchRequest {
    pub query: String,
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