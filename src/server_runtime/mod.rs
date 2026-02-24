/*! This module contains the async runtime for the REST API.
 */

use std::convert::Infallible;
use warp::{Filter, Rejection, Reply, http::Method, http::StatusCode};

mod handler;
pub mod types;

async fn handle_rejection(err: Rejection) -> std::result::Result<impl Reply, Infallible> {
    let (code, message) = if err.is_not_found() {
        (StatusCode::NOT_FOUND, "Not Found".to_string())
    } else if err.find::<warp::reject::PayloadTooLarge>().is_some() {
        (StatusCode::BAD_REQUEST, "Payload too large".to_string())
    } else {
        /* this might have to be changed to something more robust */
        eprintln!("unhandled error: {:?}", err);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Internal Server Error".to_string(),
        )
    };

    Ok(warp::reply::with_status(message, code))
}

/** Entrypoint for the server runtime loop */
pub async fn run_server(port: u16, max_file_size: Option<u64>) {
    let cors = warp::cors()
        .allow_any_origin()
        .allow_header("content-type")
        .allow_methods(&[Method::PUT, Method::GET, Method::POST, Method::DELETE]);
    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let hello = warp::path!("hello" / String).map(|name| format!("Hello, {}!", name));
    let post_submission = warp::post()
        .and(warp::path("submit"))
        .and(
            warp::multipart::form().max_length(if let Some(x) = max_file_size {
                x
            } else {
                5_000_000
            }),
        )
        .and(warp::path::end())
        .map(|v| format!("got something"));

    let route = hello
        .or(post_submission)
        .with(cors)
        .recover(handle_rejection);
    warp::serve(route).run(([127, 0, 0, 1], port)).await;
}
