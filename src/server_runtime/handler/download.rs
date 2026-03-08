/*!
Module that provides file downloading capabilities through HTTP
 */
use crate::{
    adapter::fs::asynchronous::ensure_parent,
    server_runtime::error::RuntimeError,
    types::{
        assignment_id::AssignmentId, job::Job, submission_hash::SubmissionHash,
        submission_type::SubmissionType,
    },
};
use bytes::BufMut;
use futures::TryStreamExt;
use std::{path::Path, sync::Arc};
use uuid::Uuid;
use warp::multipart::Part;

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

    ensure_parent(&target).await.map_err(|e| {
        eprint!("error with parent directory: {}", e);
        RuntimeError::DownloadFailure
    })?;
    tokio::fs::write(target, value).await.map_err(|e| {
        eprint!("error writing file: {}", e);
        RuntimeError::DownloadFailure
    })?;
    Ok(())
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

fn get_file_extension(
    allowed_types: Arc<Vec<(&str, &str)>>,
    content_type: Option<&str>,
    filename: Option<&str>,
) -> Result<String, RuntimeError> {
    content_type
        .and_then(|x| filter_files_return_suffix(&allowed_types, x))
        .map(|v| v.to_owned())
        .ok_or(RuntimeError::InvalidFiletype)
        .or_else(|e| {
            filename
                .and_then(|v| match SubmissionType::try_from(v) {
                    Ok(submission_type) => Some(submission_type.extension().to_owned()),
                    Err(_) => None,
                })
                .ok_or_else(|| e)
        })
}

/**
Function that implements the download file stage of the POST submission
endpoint.
 */
pub async fn download_file_sequence<P: AsRef<Path>>(
    assign_handle: &str,
    download_dir: P,
    allowed_types: Arc<Vec<(&str, &str)>>,
    p: Part,
) -> Result<Job, RuntimeError> {
    let file_ending = get_file_extension(allowed_types, p.content_type(), p.filename())?;
    let job_uuid = Uuid::new_v4();
    let file_name = format!("{}.{}", &job_uuid, file_ending);
    let target_path = download_dir.as_ref().join(&file_name);
    download_file_part(&target_path, p).await.map_err(|e| {
        eprintln!("{}", e.to_string());
        e
    })?;
    let path_ref = target_path.clone();
    let file_hash = tokio::task::spawn_blocking(move || {
        SubmissionHash::try_from(path_ref.as_path()).or_else(|_| Err(RuntimeError::HashingFailure))
    })
    .await
    .map_err(|_| RuntimeError::HashingFailure)??;
    Ok(Job::new(
        job_uuid,
        AssignmentId::new(assign_handle.to_owned()),
        target_path,
        file_hash,
    ))
}
