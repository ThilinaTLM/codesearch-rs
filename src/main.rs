use std::sync::Arc;

mod engine;
mod utils;
mod config;
mod api;

#[tokio::main]
async fn main() {
    env_logger::init();

    let config_path = "config.yaml";
    let config = Arc::new(config::load_config(config_path).unwrap());
    utils::misc::init_data_dir(&config.index.data_dir).unwrap();

    let engine = engine::FileSearchEngine::new(config.clone())
        .unwrap();
    api::start_api(config.clone(), engine).await;
}