mod api;
mod search;
mod utils;
mod config;

#[tokio::main]
async fn main() {
    let config = config::load_config("config.yaml").unwrap();
    let engine = search::FileSearchEngine::new(&config).unwrap();
    engine.initialize().await.unwrap();
    api::run_api().await;
}