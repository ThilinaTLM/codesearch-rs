use std::fs;

use serde::Deserialize;
use serde_yaml;

pub(crate) fn load_config(file_path: &str) -> Result<Config, serde_yaml::Error> {
    let contents = fs::read_to_string(file_path)
        .expect("Failed to read YAML file");
    let config: Config = serde_yaml::from_str(&contents)?;
    config.validate()
        .expect("Invalid config");
    Ok(config)
}

trait Validatable {
    fn validate(&self) -> Result<(), String>;
}

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct Config {
    pub(crate) repos: Vec<Repo>,
    pub(crate) indexer: Indexer,
}

impl Validatable for Config {
    fn validate(&self) -> Result<(), String> {
        if self.repos.is_empty() {
            return Err("No repos found in config".to_string());
        }
        for repo in &self.repos {
            repo.validate()?;
        }
        Ok(())
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct Repo {
    pub(crate) name: String,
    #[serde(rename = "type")]
    pub(crate) type_: String,
    pub(crate) path: String,
    pub(crate) skip_patterns: Vec<String>,
    pub(crate) allowed_file_extensions: Vec<String>,
}

impl Validatable for Repo {
    fn validate(&self) -> Result<(), String> {
        if self.name.is_empty() {
            return Err("Repo name is empty".to_string());
        }

        if self.path.is_empty() {
            return Err("Repo path is empty".to_string());
        }
        if self.type_ != "fs" {
            return Err("Repo type is not fs".to_string());
        }

        if self.type_.is_empty() {
            return Err("Repo type is empty".to_string());
        }

        Ok(())
    }
}


#[derive(Debug, Deserialize, Clone)]
pub struct Indexer {
    pub(crate) use_temporary_index: bool,
    pub(crate) index_path: Option<String>,
    pub(crate) force_reindex: bool,
}

impl Validatable for Indexer {
    fn validate(&self) -> Result<(), String> {
        if self.use_temporary_index && self.index_path.is_some() {
            return Err("Cannot use temporary index and specify index path".to_string());
        }
        if self.index_path.is_none() && !self.use_temporary_index {
            return Err("Must specify index path or use temporary index".to_string());
        }
        Ok(())
    }
}