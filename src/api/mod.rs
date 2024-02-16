use std::convert::Infallible;
use std::sync::Arc;

#[cfg(not(debug_assertions))]
use include_dir::{Dir, include_dir};

use warp::{self, Filter, Reply};
use warp::http::Method;

use crate::api::models::{HealthCheckResponse, IndexForm, SearchForm, StdResponse};
use crate::config::Index;
use crate::engine::{FileSearchEngine, SearchEngine, SearchOptions};

mod models;

#[cfg(not(debug_assertions))]
static WEB_DIR: Option<Dir> = include_dir!("$CARGO_MANIFEST_DIR/web/dist");

pub async fn start_api(engine: FileSearchEngine) {
    log::info!("Starting API server...");

    let engine_arc = Arc::new(engine);

    let cors_filter = warp::cors()
        .allow_any_origin()
        .allow_methods(vec![Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers(vec!["content-type"])
        .build();

    #[cfg(not(debug_assertions))]
    let web_static_route = warp::path("ui")
        .and(warp::path::tail())
        .and_then(move |tail: warp::path::Tail| {
            let file_path = format!("{}{}", WEB_DIR.expect("couldn't find ui build").path().display(), tail.as_str());
            log::info!("Serving static file: {}", file_path);
            async move {
                match WEB_DIR.expect("couldn't find ui build").get_file(&file_path) {
                    Some(file) => {
                        let mime_type = mime_guess::from_path(&file_path).first_or_octet_stream().to_string();
                        Ok::<_, Infallible>(warp::reply::with_header(
                            warp::reply::with_status(file.contents(), warp::http::StatusCode::OK),
                            "content-type",
                            mime_type,
                        ).into_response())
                    }
                    None => {
                        log::error!("File not found: {}", file_path);
                        Ok::<_, Infallible>(warp::reply::with_status(
                            "File not found",
                            warp::http::StatusCode::NOT_FOUND,
                        ).into_response())
                    }
                }
            }
        });

    #[cfg(debug_assertions)]
    let web_static_route = warp::path("ui")
        .and(warp::path::tail())
        .and_then(move |_: warp::path::Tail| {
            async move {
                log::warn!("Embedded UI not available in debug mode");
                Ok::<_, Infallible>(warp::reply::with_status(
                    "Embedded UI not available in debug mode",
                    warp::http::StatusCode::NOT_FOUND,
                ).into_response())
            }
        });

    let health_route = warp::path("health")
        .and(warp::get())
        .map(|| {
            log::info!("Received health check request");
            let start_time = std::time::Instant::now();
            let response: StdResponse<HealthCheckResponse> = StdResponse {
                data: Some(HealthCheckResponse {
                    status: "ok".to_string(),
                }),
                error: None,
                time_taken: Some(start_time.elapsed().as_millis() as u64),
            };
            warp::reply::json(&response)
        })
        .with(cors_filter.clone());

    let engine_arc_clone = engine_arc.clone();
    let search_route = warp::path("search")
        .and(warp::post())
        .and(warp::body::json())
        .and(warp::any().map(move || engine_arc_clone.clone()))
        .and_then(move |request: SearchForm, engine: Arc<FileSearchEngine>| async move {
            log::info!("Received engine request: {:?}", request.query);
            let start_time = std::time::Instant::now();
            let query = request.query;
            let limit = request.limit.unwrap_or(10);
            let results = engine.search(SearchOptions {
                query,
                limit,
            }).await;
            match results {
                Ok(results) => {
                    log::info!("Search successful, returning results");
                    let response = StdResponse {
                        data: Some(results),
                        error: None,
                        time_taken: Some(start_time.elapsed().as_millis() as u64),
                    };
                    Ok::<_, warp::Rejection>(warp::reply::json(&response))
                }
                Err(err) => {
                    log::error!("Search failed: {:?}", err);
                    let response = StdResponse::<Vec<u8>> { // Assuming no data in case of error
                        data: None,
                        error: Some(err.to_string()),
                        time_taken: Some(start_time.elapsed().as_millis() as u64),
                    };
                    Ok::<_, warp::Rejection>(warp::reply::json(&response))
                }
            }
        })
        .with(cors_filter.clone());

    let engine_arc_clone = engine_arc.clone();
    let create_index_route = warp::path("create_index")
        .and(warp::post())
        .and(warp::body::json())
        .and(warp::any().map(move || engine_arc_clone.clone()))
        .and_then(move |request: IndexForm, engine: Arc<FileSearchEngine>| async move {
            log::info!("Received create index request");
            let start_time = std::time::Instant::now();

            let repo_name = request.repo_name;
            let _ = request.force_reindex;

            let result = engine.index_repo(repo_name).await;
            match result {
                Ok(_) => {
                    log::info!("Index creation successful");
                    let response = StdResponse::<Vec<u8>> { // Assuming no data in case of error
                        data: None,
                        error: None,
                        time_taken: Some(start_time.elapsed().as_millis() as u64),
                    };
                    Ok::<_, warp::Rejection>(warp::reply::json(&response))
                }
                Err(err) => {
                    log::error!("Index creation failed: {:?}", err);
                    let response = StdResponse::<Vec<u8>> { // Assuming no data in case of error
                        data: None,
                        error: Some(err.to_string()),
                        time_taken: Some(start_time.elapsed().as_millis() as u64),
                    };
                    Ok::<_, warp::Rejection>(warp::reply::json(&response))
                }
            }
        })
        .with(cors_filter.clone());

    let get_repo_list_route = warp::path("get_repo_list")
        .and(warp::get())
        .and(warp::any().map(move || engine_arc.clone()))
        .and_then(move |engine: Arc<FileSearchEngine>| async move {
            log::info!("Received get repo list request");
            let start_time = std::time::Instant::now();
            let response = StdResponse {
                data: Some(engine.get_repo_list()),
                error: None,
                time_taken: Some(start_time.elapsed().as_millis() as u64),
            };
            Ok::<_, warp::Rejection>(warp::reply::json(&response))
        })
        .with(cors_filter.clone());

    let api_route = warp::path("api")
        .and(health_route.or(search_route));

    let routes = web_static_route.or(api_route);
    log::info!("API server running on http://127.0.0.1:3030");
    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030)).await;
}

