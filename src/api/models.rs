use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::engine::{RepoInfo, ResultItem};

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

#[derive(Serialize, Deserialize)]
pub struct RepoDto {
    pub(crate) name: String,
    pub(crate) last_indexed_time: Option<u64>,
    pub(crate) number_of_indexed_files: Option<u64>,
    pub(crate) indexing_status: String,
    pub(crate) path: String,
}

impl From<&RepoInfo> for RepoDto {
    fn from(repo_info: &RepoInfo) -> Self {
        RepoDto {
            name: repo_info.name.clone(),
            last_indexed_time: repo_info.last_indexed_time,
            number_of_indexed_files: repo_info.number_of_indexed_files,
            indexing_status: repo_info.indexing_status.clone(),
            path: repo_info.path.clone(),
        }
    }
}