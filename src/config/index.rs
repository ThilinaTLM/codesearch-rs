use serde::Deserialize;
use validator::Validate;
use crate::utils::validators::validate_path_exists;

#[derive(Debug, Deserialize, Clone, Validate)]
pub struct Index {
    #[validate(custom = "validate_path_exists")]
    #[serde(default = "default_data_dir")]
    pub(crate) data_dir: String,

    #[validate(length(min = 1))]
    #[serde(default = "default_index_dir_name")]
    pub(crate) index_dir_name: String,

    #[validate(length(min = 1))]
    #[serde(default = "default_metadata_dir_name")]
    pub(crate) metadata_dir_name: String,
}

impl Index {
    pub(crate) fn index_dir(&self) -> String {
        std::path::Path::new(&self.data_dir)
            .join(&self.index_dir_name)
            .to_str()
            .unwrap()
            .to_string()
    }

    pub(crate) fn metadata_dir(&self) -> String {
        std::path::Path::new(&self.data_dir)
            .join(&self.metadata_dir_name)
            .to_str()
            .unwrap()
            .to_string()
    }
}


fn default_data_dir() -> String {
    const DEFAULT_DATA_DIR_NAME: &str = "searchcode-rs";
    dirs::data_local_dir()
        .unwrap()
        .join(DEFAULT_DATA_DIR_NAME)
        .to_str()
        .unwrap()
        .to_string()
}

fn default_index_dir_name() -> String {
    return "index".to_string()
}

fn default_metadata_dir_name() -> String {
    return "metadata".to_string()
}