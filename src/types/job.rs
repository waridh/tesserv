/*!
Module with the model of the job.
 */

use super::assignment_id::AssignmentId;
use std::path::{Path, PathBuf};
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct Job {
    job_id: Uuid,
    assignment_id: AssignmentId,
    submission_location: PathBuf,
}

impl Job {
    pub fn new(job_id: Uuid, assignment_id: AssignmentId, submission_location: PathBuf) -> Self {
        Self {
            job_id,
            assignment_id,
            submission_location,
        }
    }

    pub fn get_job_id(&self) -> &Uuid {
        &self.job_id
    }

    pub fn get_assignment_id(&self) -> &AssignmentId {
        &self.assignment_id
    }

    pub fn get_submission_location(&self) -> &Path {
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
            self.get_submission_location().display()
        )
    }
}
