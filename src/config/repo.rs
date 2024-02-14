use serde::Deserialize;
use validator::{Validate, ValidationError};
use crate::utils::validators::{validate_string_in_patterns, validate_path_exists};

#[derive(Debug, Deserialize, Clone, Validate)]
pub struct Repo {
    #[validate(length(min = 1))]
    pub(crate) name: String,

    #[validate(custom = "validate_repo_type")]
    #[serde(rename = "type")]
    pub(crate) type_: String,

    #[validate(custom = "validate_path_exists")]
    pub(crate) path: String,

    #[serde(default = "default_skip_dir_patterns")]
    pub(crate) skip_dir_patterns: Vec<String>,

    #[validate(length(min = 1))]
    #[serde(default = "default_include_file_extensions")]
    pub(crate) include_file_extensions: Vec<String>,
}

const VALID_REPO_TYPES_STRINGS: &[&str] = &["fs"];

fn validate_repo_type(value: &str) -> Result<(), ValidationError> {
    validate_string_in_patterns(value, VALID_REPO_TYPES_STRINGS)
}


fn default_skip_dir_patterns() -> Vec<String> {
    vec![
        // Version control directories
        ".git".to_string(), ".hg".to_string(), ".svn".to_string(),

        // Build and dependency directories
        "node_modules".to_string(), "target".to_string(), "dist".to_string(),
        "build".to_string(), "out".to_string(), "bin".to_string(),
        "obj".to_string(),

        // Package manager directories
        "vendor".to_string(), "bower_components".to_string(),
        "jspm_packages".to_string(), "elm-stuff".to_string(),

        // IDE and editor configuration directories
        ".idea".to_string(), ".vscode".to_string(), ".eclipse".to_string(),
        ".metadata".to_string(),

        // Temporary and log directories
        "tmp".to_string(), "temp".to_string(), "cache".to_string(),
        "log".to_string(), "logs".to_string(),
    ]
}

fn default_include_file_extensions() -> Vec<String> {
    vec![
        // Programming languages
        "c", "cc", "cpp", "cxx", "h", "hpp", "cs", "go", "java", "kt", "m", "mm", "php",
        "py", "rb", "rs", "swift",

        // Web technologies
        "html", "css", "js", "ts", "jsx", "tsx", "less", "sass", "scss",

        // Markup and data serialization
        "xml", "json", "yaml", "yml", "md", // Markdown

        // Scripting
        "sh", "bash", "ps1", // PowerShell

        // Configuration files
        "cfg", "conf", "ini", "toml",

        // Database
        "sql",
    ]
        .into_iter()
        .map(String::from)
        .collect()
}