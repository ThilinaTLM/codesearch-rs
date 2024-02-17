use std::fs;

use serde::{Deserialize, Serialize};
use serde_yaml;
use validator::Validate;

pub use index::IndexConfig;
pub use repo::RepoConfig;

use crate::utils::validators::validate_at_least_one_item;

mod repo;
mod index;
mod server;


pub(crate) fn load_config(file_path: &str) -> Result<Config, serde_yaml::Error> {
    let contents = fs::read_to_string(file_path)
        .expect("Failed to read YAML file");
    let config: Config = serde_yaml::from_str(&contents)?;
    Ok(config)
}

#[derive(Debug, Serialize, Deserialize, Clone, Validate)]
pub(crate) struct Config {
    #[validate]
    #[serde(default)]
    pub(crate) server: server::ServerConfig,

    #[validate(custom = "validate_at_least_one_item")]
    pub(crate) repos: Vec<RepoConfig>,

    #[validate]
    #[serde(default)]
    pub(crate) index: IndexConfig,
}