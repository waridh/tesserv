/*! Module that provides handlers for the different endpoints
 */

use super::error::RuntimeError;
use crate::{
    adapter::{file_system::ensure_parent, job_portal},
    types::{assignment_id::AssignmentId, job::Job},
};
use bytes::BufMut;
use futures::{StreamExt, TryStreamExt};
use std::{path::Path, sync::Arc};
use uuid::Uuid;
use warp::{
    Reply,
    http::StatusCode,
    multipart::{FormData, Part},
};

pub async fn run_test() {}

/**
Function that builds the
 */
async fn download_file_part<P: AsRef<Path>>(target: P, p: Part) -> Result<(), RuntimeError> {
    let value = p
        .stream()
        .try_fold(Vec::new(), |mut vec, data| {
            vec.put(data);
            async move { Ok(vec) }
        })
        .await
        .map_err(|e| {
            eprintln!("reading file error: {}", e);
            RuntimeError::DownloadFailure
        })?;

    ensure_parent(&target).map_err(|e| {
        eprint!("error with parent directory: {}", e);
        RuntimeError::DownloadFailure
    })?;
    tokio::fs::write(target, value).await.map_err(|e| {
        eprint!("error writing file: {}", e);
        RuntimeError::DownloadFailure
    })?;
    Ok(())
}

/**
Function that implements the download file stage of the POST submission
endpoint.
 */
async fn download_file_sequence<P: AsRef<Path>>(
    download_dir: P,
    allowed_types: Arc<Vec<(&str, &str)>>,
    p: Part,
) -> Result<Job, RuntimeError> {
    let content_type = p.content_type();
    let file_ending = content_type
        .and_then(|x| filter_files_return_suffix(&allowed_types, x))
        .ok_or_else(|| {
            eprintln!("invalid file type");
            RuntimeError::InvalidFiletype
        })?;
    let job_uuid = Uuid::new_v4();
    let file_name = format!("{}.{}", &job_uuid, file_ending);
    let target_path = download_dir.as_ref().join(&file_name);
    download_file_part(&target_path, p)
        .await
        .map(|_| {
            Job::new(
                job_uuid,
                AssignmentId::new("something".to_string()),
                target_path,
            )
        })
        .map_err(|e| {
            eprintln!("{}", e.to_string());
            e
        })
}

fn filter_files_return_suffix<'a>(
    allowed: &[(&str, &'a str)],
    recieved_type: &str,
) -> Option<&'a str> {
    for (allowed_type, suffix) in allowed {
        if *allowed_type == recieved_type {
            return Some(suffix);
        }
    }
    None
}

/**
Handler for the submit post endpoint.

## Side effects

Will download the file sent over the web into the specified directory.
 */
pub async fn post_submit<P: AsRef<Path>>(
    mut job_portal: job_portal::JobPortal,
    download_dir: P,
    allowed_types: std::sync::Arc<Vec<(&str, &str)>>,
    form: FormData,
) -> std::result::Result<impl Reply, warp::Rejection> {
    let mut parts = form.into_stream();
    println!("handling submission");
    while let Some(Ok(p)) = parts.next().await {
        let job = download_file_sequence(&download_dir, allowed_types.clone(), p).await?;
        println!("working with job_id: {}", job);
        job_portal
            .submit(&job)
            .await
            .map_err(|_| RuntimeError::ExecutionFailure)?;
        println!("submitted job");
    }

    Ok(warp::reply::with_status("received", StatusCode::OK))
}
