use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Clone, Validate)]
pub struct ServerConfig {
    #[validate(length(min = 1))]
    #[serde(default = "default_host")]
    pub(crate) host: String,

    #[validate(range(min = 1, max = 65535))]
    #[serde(default = "default_port")]
    pub(crate) port: u16,
}

impl Default for ServerConfig {
    fn default() -> Self {
        ServerConfig {
            host: default_host(),
            port: default_port(),
        }
    }
}

fn default_host() -> String {
    "0.0.0.0".to_string()
}

fn default_port() -> u16 {
    3030
}
