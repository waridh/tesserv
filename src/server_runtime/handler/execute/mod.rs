/*!
Module that provides the facilities to execute jobs
 */

use crate::types::{assignment_config::AssignmentConfig, job::Job, submission_score};
use std::path::{Path, PathBuf};
use tokio::process;

mod setup;

#[derive(Clone, Debug)]
pub enum Error {
    SetupError(setup::Error),
    CannotFindMarkingScript((PathBuf, String)),
    MarkingScriptFailure(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Error::SetupError(e) => format!("SetupError: {}", e),
            Error::CannotFindMarkingScript((path_buf, s)) => {
                format!(
                    "Missing script: {} Failure reason: {}",
                    path_buf.display(),
                    s
                )
            }
            Error::MarkingScriptFailure(v) => format!("marking script failure: {}", v),
        };
        write!(f, "{}", value)
    }
}

impl std::error::Error for Error {}

impl From<setup::Error> for Error {
    fn from(value: setup::Error) -> Self {
        Error::SetupError(value)
    }
}

/**
Function that executes submitted jobs.
 */
pub async fn execute(assign_conf: AssignmentConfig, job: Job) -> Result<f32, Error> {
    /* TODO: Need to move the execution path to be injected. */
    let base_dir = Path::new("/tmp/tesserv");
    let workspace = setup::setup_job(base_dir, &job)
        .await
        .map_err(|e| Error::SetupError(e))?;
    let tests = assign_conf.as_ref();
    if tests.len() == 0 {
        return Ok(0.0);
    }
    let test = tests[0]
        .as_path()
        .canonicalize()
        .map_err(|e| Error::CannotFindMarkingScript((tests[0].clone(), e.to_string())))?;
    let out = process::Command::new(test)
        .current_dir(workspace.as_path())
        .output()
        .await
        .map_err(|e| Error::MarkingScriptFailure(e.to_string()))?;
    let score = submission_score::SubmissionScore::try_from(String::from_utf8_lossy(&out.stdout))
        .map_err(|e| Error::MarkingScriptFailure(e.to_string()))?;
    println!("{}", String::from_utf8_lossy(&out.stdout));

    Ok(0.0)
}
