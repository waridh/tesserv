/*!
Module with that provides the data structure for the completion
 */
use super::{
    assignment_id::AssignmentId, job, submission_hash::SubmissionHash,
    submission_score::SubmissionScore,
};
use serde::Serialize;
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, Serialize)]
pub struct CompletionReceipt {
    assignment_id: AssignmentId,
    hash: SubmissionHash,
    score: SubmissionScore,
}

impl CompletionReceipt {
    pub fn is_this(&self, (assign_id, sub_hash): (&AssignmentId, &SubmissionHash)) -> bool {
        *assign_id == self.assignment_id && *sub_hash == self.hash
    }
}

impl From<(AssignmentId, SubmissionHash, SubmissionScore)> for CompletionReceipt {
    fn from((assignment_id, hash, score): (AssignmentId, SubmissionHash, SubmissionScore)) -> Self {
        Self {
            assignment_id,
            hash,
            score,
        }
    }
}
impl From<(&AssignmentId, &SubmissionHash, &SubmissionScore)> for CompletionReceipt {
    fn from(
        (assignment_id, hash, score): (&AssignmentId, &SubmissionHash, &SubmissionScore),
    ) -> Self {
        Self::from((assignment_id.clone(), hash.clone(), score.clone()))
    }
}

impl From<(&job::Job, &SubmissionScore)> for CompletionReceipt {
    fn from((j, ss): (&job::Job, &SubmissionScore)) -> Self {
        Self::from((j.assignment_id(), j.submission_hash(), ss))
    }
}

impl Display for CompletionReceipt {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{} {} {}", self.assignment_id, self.hash, self.score)
    }
}
