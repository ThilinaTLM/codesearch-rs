use std::fs;

use serde::Deserialize;
use validator::{Validate, ValidationError};
use serde_yaml;


pub(crate) fn load_config(file_path: &str) -> Result<Config, serde_yaml::Error> {
    let contents = fs::read_to_string(file_path)
        .expect("Failed to read YAML file");
    let config: Config = serde_yaml::from_str(&contents)?;
    Ok(config)
}

#[derive(Debug, Deserialize, Clone, Validate)]
pub(crate) struct Config {
    #[validate]
    pub(crate) repos: Vec<Repo>,
    #[validate]
    pub(crate) indexer: Indexer,
}

#[derive(Debug, Deserialize, Clone, Validate)]
pub struct Repo {
    #[validate(length(min = 1))]
    pub(crate) name: String,

    #[validate(contains(pattern = "^fs"))]
    #[serde(rename = "type")]
    pub(crate) type_: String,

    #[validate(length(min = 1))]
    pub(crate) path: String,

    pub(crate) skip_patterns: Vec<String>,
    pub(crate) allowed_file_extensions: Vec<String>,
}


#[derive(Debug, Deserialize, Clone, Validate)]
pub struct Indexer {
    pub(crate) use_temporary_index: bool,
    pub(crate) index_path: Option<String>,
    pub(crate) force_reindex: bool,
}