use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::engine::{RepoInfo, SearchResultItem};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StdResponse<T> where T: Serialize {
    pub data: Option<T>,
    pub error: Option<String>,
    pub time_taken: Option<u64>,
}


#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchForm {
    pub query: String,
    pub limit: Option<usize>,
}

#[derive(Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct IndexForm {
    #[validate(length(min = 1))]
    pub(crate) repo_name: String,

    #[serde(default)]
    pub(crate) force_reindex: bool,
}

#[derive(Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct FileContentForm {
    #[validate(length(min = 1))]
    pub(crate) path: String,

    #[validate(length(min = 1))]
    pub(crate) repo_name: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HealthCheckResponse {
    pub status: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RepoDto {
    pub name: String,
    pub last_indexed_time: Option<u64>,
    pub number_of_indexed_files: Option<u64>,
    pub indexing_status: String,
    pub path: String,
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

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResultItemDto {
    pub _score: f32,
    pub repo_name: String,
    pub repo_path: String,
    pub repo_type: String,
    pub file_name: String,
    pub file_path: String,
    pub file_ext: String,
    pub file_size: u64,
    pub last_updated: chrono::DateTime<chrono::Utc>,
}

impl From<&SearchResultItem> for ResultItemDto {
    fn from(item: &SearchResultItem) -> Self {
        ResultItemDto {
            _score: item._score,
            repo_name: item.data.repo_name.clone(),
            repo_path: item.data.repo_path.clone(),
            repo_type: item.data.repo_type.clone(),
            file_name: item.data.file_name.clone(),
            file_path: item.data.file_path.clone(),
            file_ext: item.data.file_ext.clone(),
            file_size: item.data.file_size,
            last_updated: item.data.last_updated.clone(),
        }
    }
}