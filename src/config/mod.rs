use std::fs;

use serde::{Deserialize, Serialize};
use serde_yaml;
use validator::Validate;
use crate::utils::validators::validate_at_least_one_item;

pub use repo::Repo;
pub use index::Index;

mod repo;
mod index;


pub(crate) fn load_config(file_path: &str) -> Result<Config, serde_yaml::Error> {
    let contents = fs::read_to_string(file_path)
        .expect("Failed to read YAML file");
    let config: Config = serde_yaml::from_str(&contents)?;
    Ok(config)
}

#[derive(Debug, Serialize, Deserialize, Clone, Validate)]
pub(crate) struct Config {
    #[validate(custom = "validate_at_least_one_item")]
    pub(crate) repos: Vec<Repo>,

    #[validate]
    #[serde(default)]
    pub(crate) index: Index,
}