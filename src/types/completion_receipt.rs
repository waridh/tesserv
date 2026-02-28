use super::{
    assignment_id::AssignmentId, submission_hash::SubmissionHash, submission_score::SubmissionScore,
};
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug)]
pub struct CompletionReceipt {
    assignment_id: AssignmentId,
    hash: SubmissionHash,
    score: SubmissionScore,
}

impl CompletionReceipt {
    pub fn entry_string(&self) -> String {
        format!("{} {}", self.hash, self.score)
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

impl Display for CompletionReceipt {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{} {} {}", self.assignment_id, self.hash, self.score)
    }
}
