/*!
Module that provides the facilities to execute jobs
 */

use crate::types::{
    assignment_config::AssignmentConfig, completion_receipt::CompletionReceipt, job::Job,
    submission_score::SubmissionScore,
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

async fn run_single<P, Q>(test_script: P, workspace: Q) -> Result<SubmissionScore, Error>
where
    P: AsRef<Path>,
    Q: AsRef<Path>,
{
    let test_ref = test_script.as_ref();
    let test = test_ref
        .canonicalize()
        .map_err(|e| Error::CannotFindMarkingScript((test_ref.to_owned(), e.to_string())))?;
    let out = process::Command::new(&test)
        .current_dir(workspace.as_ref())
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
    SubmissionScore::try_from(&test_out).map_err(|e| {
        eprintln!(
            "failed to parse:\n{}\nwith the following reason: {}",
            &test_out, e
        );
        Error::ScoreParsingError
    })
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
    let score_iter: Vec<_> = tests
        .iter()
        .map(async |v| run_single(v.as_path(), workspace.as_path()).await)
        .collect();
    let mut score = SubmissionScore::new(0, 0);
    for work in score_iter {
        let res = work.await?;
        score = score + res;
    }
    // let score = run_single(tests[0].as_path(), workspace.as_path()).await?;
    println!("{}", score);

    Ok(CompletionReceipt::from((&job, &score)))
}
