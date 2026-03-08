/*! module that provides async access to a log store.
 */

use super::fs::standard::ensure_parent;
use crate::types::{
    assignment_id::AssignmentId, completion_receipt::CompletionReceipt,
    submission_hash::SubmissionHash, submission_score::SubmissionScore,
};
use std::{
    env,
    fs::File,
    io::{Read, Write},
    path::Path,
    path::PathBuf,
    sync::Arc,
};
use tokio::sync::Mutex;

fn classic_xdg_data_home_from_home<P>(home: P) -> PathBuf
where
    P: AsRef<Path>,
{
    home.as_ref().join(".local").join("share")
}

fn application_data_home() -> Option<PathBuf> {
    env::var("XDG_DATA_HOME")
        .and_then(|p| Ok(PathBuf::from(p)))
        .ok()
        .or_else(|| {
            env::home_dir()
                .map(|p| classic_xdg_data_home_from_home(p))
                .map(|p| p.join("tesserv"))
        })
        .or_else(|| {
            env::var("HOME")
                .map(|v| PathBuf::from(v))
                .ok()
                .map(|p| classic_xdg_data_home_from_home(p))
                .map(|p| p.join("tesserv"))
        })
}

#[derive(Clone, Debug)]
pub enum LogStoreError {
    LogStoreInitError(String),
    CannotLockFile,
    CannotUnlockFile,
    WriteFailure(String),
    ReadFailure(String),
}
impl std::fmt::Display for LogStoreError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            LogStoreError::LogStoreInitError(s) => format!("could not initialize log store: {s}"),
            LogStoreError::CannotLockFile => format!("unable to lock the log store file"),
            LogStoreError::CannotUnlockFile => format!("unable to unlock the file "),
            LogStoreError::WriteFailure(s) => format!("write failure: {s}"),
            LogStoreError::ReadFailure(s) => format!("failed to read from log file: {s}"),
        };
        write!(f, "{value}")
    }
}

impl std::error::Error for LogStoreError {}

#[derive(Clone, Debug)]
pub struct LogStore {
    store_loc: Arc<Mutex<File>>,
}

impl LogStore {
    pub fn try_new() -> Result<Self, LogStoreError> {
        let default_data_dir = application_data_home().ok_or(LogStoreError::LogStoreInitError(
            "could not create default data location".to_owned(),
        ))?;
        let default_path = default_data_dir.join("tesserv_store.txt");
        LogStore::try_from(default_path.as_path())
    }
    async fn write_all(&self, v: &[u8]) -> Result<(), LogStoreError> {
        /* NOTE: In the future, once tokio provides file locks for their fs
        module, we would like to swap to that.
         */
        let store_loc = self.store_loc.clone();
        let copy = v.to_owned();
        tokio::task::block_in_place(async move || {
            let mut file = store_loc.lock().await;
            file.lock().map_err(|_| LogStoreError::CannotLockFile)?;
            let res = file
                .write_all(&copy)
                .map_err(|e| LogStoreError::WriteFailure(e.to_string()));
            file.unlock().map_err(|_| LogStoreError::CannotUnlockFile)?;
            res
        })
        .await
    }
    async fn read_all(&self) -> Result<Vec<CompletionReceipt>, LogStoreError> {
        /* NOTE: Currently, this is an expensive implementation, but the main
        runtime does not need to execute this method */
        let store_loc = self.store_loc.clone();
        tokio::task::block_in_place(async move || -> Result<Vec<_>, LogStoreError> {
            let mut file = store_loc.lock().await;
            let mut buffer = String::new();
            file.lock().map_err(|_| LogStoreError::CannotLockFile)?;
            let _ = file
                .read_to_string(&mut buffer)
                .map_err(|e| LogStoreError::ReadFailure(e.to_string()))?;
            file.unlock().map_err(|_| LogStoreError::CannotUnlockFile)?;
            Ok(buffer
                .split("\n")
                .map(|line| line.split(" "))
                .map(|line| {
                    let mut first_three = line.take(3);
                    let assign_id = first_three.next();
                    let hash = first_three.next();
                    let score = first_three.next();
                    (assign_id, hash, score)
                })
                .filter_map(|v| {
                    if let (Some(assign), Some(hash), Some(score)) = v {
                        Some((assign, hash, score))
                    } else {
                        None
                    }
                })
                .filter_map(|(a, h, s)| {
                    let hash = SubmissionHash::from_hash_string(h.to_owned());
                    let a_id = AssignmentId::from(a);
                    let score_res = SubmissionScore::try_from(s);
                    if let Ok(score) = score_res {
                        Some((a_id, hash, score))
                    } else {
                        None
                    }
                })
                .map(|v| CompletionReceipt::from(v))
                .collect())
        })
        .await
    }

    pub async fn query_score(
        &self,
        (assign_id, sub_hash): (&AssignmentId, &SubmissionHash),
    ) -> Option<CompletionReceipt> {
        let all = self.read_all().await.ok();
        if let Some(x) = all {
            x.iter()
                .filter(|c| c.is_this((assign_id, sub_hash)))
                .map(|x| x.to_owned())
                .next()
        } else {
            None
        }
    }

    pub async fn write_receipt(&self, value: &CompletionReceipt) -> Result<(), LogStoreError> {
        self.write_all(format!("{}\n", value).as_bytes()).await
    }
}

/**
 */
impl TryFrom<&Path> for LogStore {
    type Error = LogStoreError;
    fn try_from(path: &Path) -> Result<Self, Self::Error> {
        ensure_parent(path).map_err(|e| LogStoreError::LogStoreInitError(e.to_string()))?;
        let file_res = if path.exists() {
            File::options()
                .read(true)
                .append(true)
                .create(true)
                .open(path)
        } else {
            File::options()
                .read(true)
                .write(true)
                .create(true)
                .open(path)
        };
        let file = if let Ok(x) = file_res {
            x
        } else {
            return Err(LogStoreError::LogStoreInitError(
                "could not create file".to_string(),
            ));
        };
        let store_loc = Arc::new(Mutex::new(file));
        Ok(LogStore { store_loc })
    }
}
