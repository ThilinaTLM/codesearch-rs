use std::convert::Infallible;
use serde::{Deserialize, Serialize};
use warp::Filter;
use warp::http::Method;

use crate::search;
use crate::search::{ResultItem};

#[derive(Deserialize)]
pub struct SearchQuery {
    query: String,
}

#[derive(Serialize)]
pub struct SearchResponse {
    results: Option<Vec<ResultItem>>,
    error: Option<String>,
}

#[derive(Serialize)]
pub struct HealthCheckResponse {
    status: String,
}

fn cors_filter() -> warp::filters::cors::Cors {
    warp::cors()
        .allow_any_origin()
        .allow_methods(vec![Method::GET, Method::POST, Method::DELETE])
        .allow_headers(vec!["content-type", "Authorization"])
        .allow_credentials(true)
        .build()
}

pub async fn run_api() {
    let search_route = warp::path("search")
        .and(warp::get())
        .and(warp::query::<SearchQuery>())
        .and(warped_engine)
        .and_then(|search_query: SearchQuery, engine: search::FileSearchEngine| async move {
            let options = search::SearchOptions {
                query: search_query.query,
            };
            match engine.search(options).await { // Make sure to await the future here
                Ok(search_results) => {
                    let response = SearchResponse {
                        results: Some(search_results.results), // Assuming SearchResult has a field `results`
                        error: None,
                    };
                    Ok::<_, Infallible>(warp::reply::json(&response))
                }
                Err(e) => {
                    let response = SearchResponse {
                        results: None,
                        error: Some(e.to_string()),
                    };
                    Ok::<_, Infallible>(warp::reply::json(&response))
                }
            }
        });

    let hello = warp::path("health")
        .and(warp::get())
        .map(|| {
            warp::reply::json(
                &HealthCheckResponse {
                    status: "ok".to_string(),
                }
            )
        });

    let routes = search_route.or(hello).with(cors_filter());

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}