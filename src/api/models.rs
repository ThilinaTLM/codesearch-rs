use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::engine::ResultItem;

#[derive(Serialize, Deserialize)]
pub struct StdResponse<T> where T: Serialize {
    pub(crate) data: Option<T>,
    pub(crate) error: Option<String>,
    pub(crate) time_taken: Option<u64>,
}


#[derive(Serialize, Deserialize)]
pub struct SearchForm {
    pub query: String,
    pub limit: Option<usize>,
}

#[derive(Serialize, Deserialize, Validate)]
pub struct IndexForm {
    #[validate(length(min = 1))]
    pub(crate) repo_name: String,

    #[serde(default)]
    pub(crate) force_reindex: bool,
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

#[derive(Serialize, Deserialize)]
pub struct IndexResponse {
    pub(crate) status: String,
    pub(crate) time_taken: u64,
}