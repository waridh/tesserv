/*!
Module that provides handlers for the different endpoints
 */

use crate::{
    adapter::{assignment_store, job_portal},
    server_runtime::error::RuntimeError,
    types::assignment_config::AssignmentConfig,
};
use futures::{StreamExt, TryStreamExt};
use std::path::Path;
use warp::{Reply, multipart::FormData};

mod download;
mod execute;

/**
Handler for the submit post endpoint.

## Side effects

Will download the file sent over the web into the specified directory.
 */
pub async fn post_submit<P: AsRef<Path>>(
    assign_handle: String,
    form: FormData,
    assign_store: assignment_store::AssignmentStore,
    mut _job_portal: job_portal::JobPortal,
    download_dir: P,
    allowed_types: std::sync::Arc<Vec<(&str, &str)>>,
) -> std::result::Result<impl Reply, warp::Rejection> {
    let assign_config = if let Some(x) = assign_store.get(assign_handle.as_str()).await {
        x
    } else {
        return Err(RuntimeError::InvalidAssignmentId(assign_handle))?;
    };
    let mut parts = form.into_stream();
    println!("handling submission");
    while let Some(Ok(p)) = parts.next().await {
        let job = download::download_file_sequence(&download_dir, allowed_types.clone(), p).await?;
        println!("working with job_id: {}", job); // TODO: Swap these to logging
        let score = execute::execute(assign_config, job).await.map_err(|e| {
            eprintln!("{}", e);
            RuntimeError::ExecutionFailure(e.to_string())
        })?;
        println!("executed job");
        return Ok(warp::reply::json(&score));
    }
    Err(RuntimeError::DownloadFailure.into())
}
