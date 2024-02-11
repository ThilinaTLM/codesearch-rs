mod search;
mod utils;
mod config;
mod api;

#[tokio::main]
async fn main() {
    env_logger::init();

    let config = config::load_config("config.yaml").unwrap();
    let engine = search::FileSearchEngine::new(&config).unwrap();
    let _ = engine.initialize().await;
    api::start_api(engine).await;
}