use std::sync::Arc;

#[cfg(not(debug_assertions))]
use include_dir::{Dir, include_dir};
use warp::reply::{Json, Response};
use warp::Reply;

use crate::api::models::{HealthCheckResponse, IndexForm, RepoDto, SearchForm, StdResponse};
use crate::engine::{FileSearchEngine, SearchEngine, SearchOptions};

#[cfg(not(debug_assertions))]
static WEB_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/web/dist");

pub fn health_route_handler() -> warp::reply::Json {
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
}

pub async fn web_ui_route_handler(tail: warp::path::Tail) -> Result<Response, warp::Rejection> {
    log::info!("Received request for UI: {}", tail.as_str());

    #[cfg(debug_assertions)]
    return {
        log::warn!("Embedded UI not available in debug mode");
        let reply = warp::reply::with_status(
            "Embedded UI not available in debug mode",
            warp::http::StatusCode::NOT_FOUND,
        );
        Ok(reply.into_response())
    };

    #[cfg(not(debug_assertions))]
    {
        log::info!("Received request for UI: {}", tail.as_str());

        let web_dir_arc = WEB_DIR.clone();
        let file_path = format!("{}{}", web_dir_arc.path().display(), tail.as_str());
        log::info!("Serving static file: {}", file_path);

        let web_dir = web_dir_arc.clone();
        match web_dir.get_file(&file_path) {
            Some(file) => {
                let mime_type = mime_guess::from_path(&file_path).first_or_octet_stream().to_string();
                let reply = warp::reply::with_header(
                    warp::reply::with_status(file.contents(), warp::http::StatusCode::OK),
                    "content-type",
                    mime_type,
                ).into_response();
                Ok(reply)
            }
            None => {
                log::error!("File not found: {}", file_path);
                Err(warp::reject::not_found())
            }
        }
    }
}

pub async fn search_route_handler(request: SearchForm, engine: Arc<FileSearchEngine>) -> Result<Json, warp::Rejection> {
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
}

pub async fn create_index_route_handler(request: IndexForm, engine: Arc<FileSearchEngine>) -> Result<Json, warp::Rejection> {
    log::info!("Received create index request");
    let start_time = std::time::Instant::now();

    let repo_name = request.repo_name;

    tokio::spawn(async move {
        let _ =  engine.index_repo(repo_name).await;
    });

    log::info!("Index creation triggered");
    let response = StdResponse {
        data: Some("Index creation triggered".to_string()),
        error: None,
        time_taken: Some(start_time.elapsed().as_millis() as u64),
    };
    Ok::<_, warp::Rejection>(warp::reply::json(&response))
}

pub async fn get_repo_list_route_handler(engine: Arc<FileSearchEngine>) -> Result<Json, warp::Rejection> {
    log::info!("Received get repo list request");
    let start_time = std::time::Instant::now();

    let repo_info = engine.get_repo_list().await.unwrap();
    let repo_dtos: Vec<RepoDto> = repo_info.iter().map(|repo| {
        repo.into()
    }).collect();

    let response = StdResponse {
        data: Some(repo_dtos),
        error: None,
        time_taken: Some(start_time.elapsed().as_millis() as u64),
    };
    Ok::<_, warp::Rejection>(warp::reply::json(&response))
}