/*!
Module with the model of the job.
 */

use super::{assignment_id::AssignmentId, job_id::JobId};
use std::path::{Path, PathBuf};
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct Job {
    job_id: JobId,
    assignment_id: AssignmentId,
    submission_location: PathBuf,
}

impl Job {
    pub fn new<U: AsRef<Uuid>>(
        job_id: U,
        assignment_id: AssignmentId,
        submission_location: PathBuf,
    ) -> Self {
        let job_id_aux = JobId::from(job_id.as_ref());
        Self {
            job_id: job_id_aux,
            assignment_id,
            submission_location,
        }
    }

    pub fn job_id(&self) -> &JobId {
        &self.job_id
    }

    pub fn assignment_id(&self) -> &AssignmentId {
        &self.assignment_id
    }

    pub fn submission_location(&self) -> &Path {
        self.submission_location.as_path()
    }
}

impl std::fmt::Display for Job {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "(:job_id {} :assignment_id {} :submission_location {})",
            self.job_id,
            self.assignment_id,
            self.submission_location().display()
        )
    }
}

impl AsRef<JobId> for Job {
    fn as_ref(&self) -> &JobId {
        self.job_id()
    }
}
