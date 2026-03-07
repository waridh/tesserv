use crate::types::job::Job;
use std::path::{Path, PathBuf};
use tokio::{fs, process};

#[derive(Clone, Debug)]
pub enum Error {
    IOError(String),
    UnpackError(String),
    TeardownError(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Self::IOError(s) => s.to_owned(),
            Self::UnpackError(s) => format!("UnpackError {}", s),
            Self::TeardownError(s) => format!("failed to teardown: {}", s),
        };
        write!(f, "{}", value)
    }
}

pub fn workspace_path<P>(base_path: P, job: &Job) -> PathBuf
where
    P: AsRef<Path>,
{
    base_path.as_ref().join(job.job_id().to_string())
}

/**
helper function that creates a directory.
 */
async fn build_unique_dir<P>(dir: P) -> Result<(), Error>
where
    P: AsRef<Path>,
{
    let target_dir = dir.as_ref();
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
async fn unpack_package<P, Q>(from: P, to: Q) -> Result<(), Error>
where
    P: AsRef<Path>,
    Q: AsRef<Path>,
{
    let to_path = to.as_ref();
    if !to_path.exists() {
        return Err(Error::IOError("target path does not exist".to_string()));
    } else if !to_path.is_dir() {
        return Err(Error::IOError("target path is not a directory".to_string()));
    }
    process::Command::new("tar")
        .arg("-xf")
        .arg(format!("{}", from.as_ref().display()))
        .arg("-C")
        .arg(format!("{}", to_path.display()))
        .kill_on_drop(true)
        .status()
        .await
        .map_err(|e| Error::UnpackError(e.to_string()))
        .and_then(|v| {
            if v.success() {
                Ok(())
            } else {
                let msg = if let Some(x) = v.code() {
                    format!("unpack failed with code: {}", x)
                } else {
                    String::from("unpack failed without code")
                };
                Err(Error::UnpackError(msg))
            }
        })
}

/**
function that sets up the workspace for the job

# Arguments

- `base_path` is the base path for the tesserv application

# Returns

A result where the Ok case contains the working directory of the system.
 */
pub async fn setup_job<P>(base_path: P, job: &Job) -> Result<PathBuf, Error>
where
    P: AsRef<Path>,
{
    let workspace = workspace_path(base_path.as_ref(), job);
    build_unique_dir(workspace.as_path()).await?;
    unpack_package(job.submission_location(), workspace.as_path()).await?;
    Ok(workspace)
}

pub async fn teardown_job<P>(base_path: P, job: &Job) -> Result<(), Error>
where
    P: AsRef<Path>,
{
    let workspace = workspace_path(base_path.as_ref(), job);
    if workspace.exists() && workspace.is_dir() {
        fs::remove_dir_all(&workspace)
            .await
            .map_err(|e| Error::TeardownError(e.to_string()))?;
    }
    Ok(())
}
