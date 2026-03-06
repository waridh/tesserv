/*! This module contains the async runtime for the REST API.
 */

use crate::adapter::job_portal::JobPortal;
use warp::{
    Filter, Reply,
    http::{Method, StatusCode},
};

mod error;
mod handler;

async fn hello_fun(name: String) -> Result<impl Reply, warp::Rejection> {
    Ok(warp::reply::with_status(
        format!("hello, {}!", name),
        StatusCode::OK,
    ))
}

/** Entrypoint for the server runtime loop.
Denotes the different routes that are accepted in the server.
 */
pub async fn run_server(port: u16, max_file_size: Option<u64>) {
    /* Resource initialization */

    let job_portal = JobPortal::new();
    // TODO: Convert this to just the archive types
    let allowed_types = std::sync::Arc::new(vec![
        ("application/pdf", "pdf"),
        ("image/png", "png"),
        ("image/jpeg", "jpeg"),
    ]);
    let registered_max_file_size = max_file_size.unwrap_or(10_000_000);
    let download_dir_filter = warp::any().map(|| std::path::Path::new("/tmp/test-files2/"));
    let allowed_types_filter = warp::any().map(move || allowed_types.clone());
    let job_portal_filter = warp::any().map(move || job_portal.clone());

    let cors = warp::cors()
        .allow_any_origin()
        .allow_header("content-type")
        .allow_methods(&[Method::PUT, Method::GET, Method::POST, Method::DELETE]);

    // TODO: Remove the hello world route.
    let hello = warp::path!("hello" / String).and_then(hello_fun);
    let post_submission = warp::post()
        .and(job_portal_filter)
        .and(download_dir_filter)
        .and(allowed_types_filter)
        .and(warp::path("submit"))
        .and(warp::multipart::form().max_length(registered_max_file_size))
        .and(warp::path::end())
        .and_then(handler::post_submit);

    let route = hello
        .or(post_submission)
        .with(cors)
        .recover(error::return_error);
    warp::serve(route).run(([127, 0, 0, 1], port)).await;
}
