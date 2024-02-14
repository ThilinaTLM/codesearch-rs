use std::fs;

use serde::Deserialize;
use serde_yaml;
use validator::Validate;

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

#[derive(Debug, Deserialize, Clone, Validate)]
pub(crate) struct Config {
    #[validate]
    pub(crate) repos: Vec<Repo>,
    #[validate]
    pub(crate) index: Index,
}

fn validate_at_least_one_repo(repos: &Vec<Repo>) -> Result<(), validator::ValidationError> {
    if repos.is_empty() {
        let mut error = validator::ValidationError::new("at_least_one_repo");
        error.message = Some("At least one repo must be provided".into());
        Err(error)
    } else {
        Ok(())
    }
}
