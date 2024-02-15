mod engine;
mod utils;
mod config;
mod api;

#[tokio::main]
async fn main() {
    env_logger::init();

    let config = config::load_config("config.yaml").unwrap();
    let engine = engine::FileSearchEngine::new(&config).unwrap();
    api::start_api(engine).await;
}