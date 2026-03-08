/*! This module contains the async runtime for the REST API.
 */

use crate::adapter::{
    assignment_store::AssignmentStore, log_store::LogStore, tesserv_config::TesservConfig,
};
use warp::{Filter, http::Method};

mod error;
mod handler;

/** Entrypoint for the server runtime loop.
Denotes the different routes that are accepted in the server.
 */
pub async fn run_server(
    port: u16,
    config: TesservConfig,
    max_file_size: Option<u64>,
) -> Result<(), String> {
    /* Resource initialization */

    let assignment_store = AssignmentStore::from(&config);
    let log_store = LogStore::try_new().map_err(|e| e.to_string())?;

    // TODO: Convert this to just the archive types
    let allowed_types = std::sync::Arc::new(vec![("application/x-tar", "tar")]);
    let registered_max_file_size = max_file_size.unwrap_or(10_000_000);
    let download_dir_filter = warp::any().map(|| std::path::Path::new("/tmp/test-files2/"));
    let allowed_types_filter = warp::any().map(move || allowed_types.clone());
    let assignment_store_filter = warp::any().map(move || assignment_store.clone());
    let log_store_filter = warp::any().map(move || log_store.clone());

    let cors = warp::cors()
        .allow_any_origin()
        .allow_header("content-type")
        .allow_methods(&[Method::PUT, Method::GET, Method::POST, Method::DELETE]);

    // TODO: Remove the hello world route.
    let post_submission = warp::post()
        .and(warp::path("submission"))
        .and(warp::path::param::<String>())
        .and(warp::multipart::form().max_length(registered_max_file_size))
        .and(warp::path::end())
        .and(assignment_store_filter)
        .and(download_dir_filter)
        .and(allowed_types_filter)
        .and(log_store_filter)
        .and_then(handler::post_submit);

    let route = post_submission.with(cors).recover(error::return_error);
    warp::serve(route).run(([127, 0, 0, 1], port)).await;
    Ok(())
}
