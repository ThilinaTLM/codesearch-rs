use std::sync::Arc;

use warp::{self, Filter};
use warp::http::Method;

use crate::config::Config;
use crate::engine::{FileSearchEngine};

mod models;
mod handlers;


pub async fn start_api(config: Arc<Config>, engine: FileSearchEngine) {
    log::info!("Starting API server...");

    let engine_arc = Arc::new(engine);

    let cors_filter = warp::cors()
        .allow_any_origin()
        .allow_methods(vec![Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers(vec!["content-type"])
        .build();

    let web_ui_route = warp::path("ui")
        .and(warp::path::tail())
        .and_then(handlers::web_ui_route_handler);

    let health_route = warp::path("health")
        .and(warp::get())
        .map(handlers::health_route_handler);

    let engine_arc_clone = engine_arc.clone();
    let search_route = warp::path("search")
        .and(warp::post())
        .and(warp::body::json())
        .and(warp::any().map(move || engine_arc_clone.clone()))
        .and_then(handlers::search_route_handler)
        .with(cors_filter.clone());

    let engine_arc_clone = engine_arc.clone();
    let create_index_route = warp::path("index")
        .and(warp::post())
        .and(warp::body::json())
        .and(warp::any().map(move || engine_arc_clone.clone()))
        .and_then(handlers::create_index_route_handler)
        .with(cors_filter.clone());

    let get_repo_list_route = warp::path("repo")
        .and(warp::get())
        .and(warp::any().map(move || engine_arc.clone()))
        .and_then(handlers::get_repo_list_route_handler)
        .with(cors_filter.clone());


    // Combine all the routes
    let routes = web_ui_route.or(
        warp::path("api").and(
            health_route
                .or(search_route)
                .or(create_index_route)
                .or(get_repo_list_route)
        )
    );

    let server_host: String = config.server.host.clone();
    let server_port = config.server.port;

    log::info!("Server started at {}:{}", server_host, server_port);
    log::info!("API available at http://{}:{}/api", server_host, server_port);
    log::info!("Web UI available at http://{}:{}/ui", server_host, server_port);

    // socket address
    let sock_addr: std::net::SocketAddr = format!("{}:{}", server_host, server_port).parse().unwrap();
    warp::serve(routes)
        .run(sock_addr)
        .await;
}

