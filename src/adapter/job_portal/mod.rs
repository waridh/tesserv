/*!
Module that contains the implementation of job submission.
 */

use crate::types::{job::Job, job_id::JobId, submission_hash::SubmissionHash};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;

mod setup;

#[derive(Clone, Debug)]
pub enum Error {
    DuplicateJob,
    MissingJob,
    IOError(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let val = match self {
            Error::DuplicateJob => "Submitted a duplicate job".to_string(),
            Error::MissingJob => "Missing target job".to_string(),
            Error::IOError(e) => format!("{}", e),
        };
        write!(f, "{}", val)
    }
}

#[derive(Clone, Debug)]
pub enum JobStatus {
    Queued,
    Active,
    Complete,
}

/**
Struct that accepts new jobs
 */
#[derive(Clone, Debug)]
pub struct JobPortal {
    active_jobs: Arc<RwLock<HashMap<JobId, (Job, JobStatus)>>>,
}

impl JobPortal {
    pub fn new() -> Self {
        let active_jobs = Arc::new(RwLock::new(HashMap::new()));
        JobPortal { active_jobs }
    }

    pub async fn status(&self, key: &JobId) -> Option<JobStatus> {
        self.get(key).await.map(|(_, status)| status)
    }

    pub async fn get(&self, key: &JobId) -> Option<(Job, JobStatus)> {
        self.active_jobs.read().await.get(key).cloned()
    }

    /**
    Mutator function that sets the status of a job
     */
    async fn update_status(&mut self, key: &JobId, value: JobStatus) -> Result<(), Error> {
        let mut store = self.active_jobs.write().await;
        if store.contains_key(key) {
            let prev_val = store.get_mut(key);
            match prev_val {
                None => Err(Error::MissingJob),
                Some((_, jstatus)) => {
                    *jstatus = value;
                    Ok(())
                }
            }
        } else {
            Err(Error::MissingJob)
        }
    }

    pub async fn contains(&self, key: &JobId) -> bool {
        self.get(key).await.is_some()
    }

    /**
    Function that submits a job to the JobPortal.
     */
    pub async fn submit(&mut self, j: &Job) -> Result<(), Error> {
        let id = j.job_id().clone();
        let mut store = self.active_jobs.write().await;
        store
            .insert(id.clone(), (j.clone(), JobStatus::Queued))
            .map(|v| {
                /* have to revert the transaction */
                store.insert(id, v);
                Err(Error::DuplicateJob)
            })
            .unwrap_or(Ok(()))
    }
}

impl Default for JobPortal {
    fn default() -> Self {
        Self::new()
    }
}
