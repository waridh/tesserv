use super::assignment_id::AssignmentId;
use std::path::PathBuf;
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct Job {
    job_id: Uuid,
    assignment_id: AssignmentId,
    submission_location: PathBuf,
}
