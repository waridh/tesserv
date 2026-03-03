/*!
Module that is responsible for setting up the workspace for the workers
 */

use super::Error;
use crate::types::job::Job;
use std::path::Path;
use tokio::{fs, process};

/**
helper function that creates a directory.
 */
async fn build_dir<P>(base_path: P, job: Job) -> Result<(), Error>
where
    P: AsRef<Path>,
{
    let target_dir = base_path.as_ref().join(job.job_id().to_string());
    if target_dir.exists() && target_dir.is_dir() {
        return Err(Error::IOError(format!(
            "{} already exists",
            target_dir.display()
        )));
    }
    fs::create_dir_all(target_dir)
        .await
        .map_err(|e| Error::IOError(e.to_string()))
}

/**
Helper function that will copy the submission to the target location
asynchronously.

NOTE: There will be a sandboxed and a non-sandboxed version of this function.
 */
async fn copy_submission<P: AsRef<Path>>(from: P, to: P) -> Result<(), Error> {
    fs::copy(from, to)
        .await
        .map(|_| ())
        .map_err(|e| Error::IOError(e.to_string()))
}

/**
Helper function that will unpack a target file in its current directory, and
then remove the archive file
 */
async fn unpack_package<P: AsRef<Path>>(from: P, to: P) -> Result<(), Error> {
    let to_path = to.as_ref();
    if !to_path.exists() {
        return Err(Error::IOError("target path does not exist".to_string()));
    } else if !to_path.is_dir() {
        return Err(Error::IOError("target path is not a directory".to_string()));
    }
    let command = process::Command::new("tar")
        .arg("-xf")
        .arg(format!("{}", from.as_ref().display()))
        .arg("-C")
        .arg(format!("{}", to_path.display()));
    Ok(())
}

/**
function that sets up the workspace for the job
 */
pub async fn setup_job<P>(base_path: P, job: Job)
where
    P: AsRef<Path>,
{
}
