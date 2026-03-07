/*!
Module that provides handlers for the different endpoints
 */

use crate::{
    adapter::job_portal, server_runtime::error::RuntimeError,
    types::assignment_config::AssignmentConfig,
};
use futures::{StreamExt, TryStreamExt};
use std::path::Path;
use warp::{Reply, http::StatusCode, multipart::FormData};

mod download;
mod execute;

/**
Handler for the submit post endpoint.

## Side effects

Will download the file sent over the web into the specified directory.
 */
pub async fn post_submit<P: AsRef<Path>>(
    mut _job_portal: job_portal::JobPortal,
    download_dir: P,
    allowed_types: std::sync::Arc<Vec<(&str, &str)>>,
    form: FormData,
) -> std::result::Result<impl Reply, warp::Rejection> {
    let mut parts = form.into_stream();
    println!("handling submission");
    while let Some(Ok(p)) = parts.next().await {
        let job = download::download_file_sequence(&download_dir, allowed_types.clone(), p).await?;
        println!("working with job_id: {}", job); // TODO: Swap these to logging
        execute::execute(
            AssignmentConfig::from(vec![Path::new("./tests/scripts/always_success.sh")]),
            job,
        )
        .await
        .map_err(|e| RuntimeError::ExecutionFailure(e.to_string()))?;
        println!("executed job");
    }

    // TODO: Swap the reply into sending the tuple of the hash and the score
    Ok(warp::reply::with_status("received", StatusCode::OK))
}
