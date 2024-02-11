use std::sync::Arc;

use warp::{self, Filter};
use warp::http::Method;

use crate::api::models::{HealthCheckResponse, SearchRequest, SearchResponse, StandardResponse};
use crate::search::{FileSearchEngine, SearchEngine, SearchOptions};

mod models;

pub async fn start_api(engine: FileSearchEngine) {
    log::info!("Starting API server...");

    let engine_arc = Arc::new(engine);

    let cors_filter = warp::cors()
        .allow_any_origin()
        .allow_methods(vec![Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers(vec!["content-type"])
        .build();

    let health_route = warp::path("health")
        .and(warp::get())
        .map(|| {
            log::info!("Received health check request");
            let response: StandardResponse<HealthCheckResponse> = StandardResponse {
                data: Some(HealthCheckResponse {
                    status: "ok".to_string(),
                }),
                error: None,
            };
            warp::reply::json(&response)
        });


    let search_route = warp::path("search")
        .and(warp::post())
        .and(warp::body::json())
        .and(warp::any().map(move || engine_arc.clone()))
        .and_then(move |request: SearchRequest, engine: Arc<FileSearchEngine>| async move {
            log::info!("Received search request: {:?}", request.query);
            let query = request.query;
            let results = engine.search(SearchOptions {
                query,
                limit: 10,
            }).await;

            match results {
                Ok(results) => {
                    log::info!("Search successful, returning results");
                    let response = StandardResponse {
                        data: Some(SearchResponse {
                            results,
                            time_taken: 0, // You might want to actually measure this.
                        }),
                        error: None,
                    };
                    Ok::<_, warp::Rejection>(warp::reply::json(&response))
                }
                Err(err) => {
                    log::error!("Search failed: {:?}", err);
                    let response = StandardResponse::<Vec<u8>> { // Assuming no data in case of error
                        data: None,
                        error: Some(err.to_string()),
                    };
                    Ok::<_, warp::Rejection>(warp::reply::json(&response))
                }
            }
        });


    let routes = health_route.or(search_route)
        .with(cors_filter);

    log::info!("API server running on http://127.0.0.1:3030");
    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030)).await;
}

