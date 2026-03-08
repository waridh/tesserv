/*!
Module that provides the facilities to execute jobs
 */

use crate::types::{
    assignment_config::AssignmentConfig, completion_receipt::CompletionReceipt, job::Job,
    submission_score,
};
use std::path::{Path, PathBuf};
use tokio::process;

mod setup;

#[derive(Clone, Debug)]
pub enum Error {
    SetupError(setup::Error),
    NoMarkingScripts,
    CannotFindMarkingScript((PathBuf, String)),
    MarkingScriptExecutionFailure(String),
    ScoreParsingError,
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
            Error::MarkingScriptExecutionFailure(v) => format!("marking script failure: {}", v),
            Error::ScoreParsingError => format!("failed to parse the marking script"),
            Error::NoMarkingScripts => {
                format!("there is no marking script in the assignment configuration")
            }
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
pub async fn execute(assign_conf: &AssignmentConfig, job: Job) -> Result<CompletionReceipt, Error> {
    /* TODO: Need to move the execution path to be injected. */
    let base_dir = Path::new("/tmp/tesserv");
    let workspace = setup::setup_job(base_dir, &job)
        .await
        .map_err(|e| Error::SetupError(e))?;
    let tests = assign_conf.as_ref();
    if tests.len() == 0 {
        return Err(Error::NoMarkingScripts);
    }
    let test = tests[0]
        .as_path()
        .canonicalize()
        .map_err(|e| Error::CannotFindMarkingScript((tests[0].clone(), e.to_string())))?;
    let out = process::Command::new(&test)
        .current_dir(workspace.as_path())
        .output()
        .await
        .map_err(|e| {
            Error::MarkingScriptExecutionFailure(format!(
                "unable to execute {} with message {}",
                test.display(),
                e
            ))
        })?;
    let test_out = String::from_utf8_lossy(&out.stdout);
    let score = submission_score::SubmissionScore::try_from(&test_out).map_err(|e| {
        eprintln!(
            "failed to parse:\n{}\nwith the following reason: {}",
            &test_out, e
        );
        Error::ScoreParsingError
    })?;
    println!("{}", score);

    Ok(CompletionReceipt::from((&job, &score)))
}
