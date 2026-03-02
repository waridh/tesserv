use crate::hasher::hash_file;
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct SubmissionHash(String);

pub enum SubmissionHashError {
    FileFailure(String),
}

impl SubmissionHash {
    pub fn try_new(path: &std::path::Path) -> Result<Self, SubmissionHashError> {
        hash_file(path).map(|s| Self(s)).map_err(|_| {
            SubmissionHashError::FileFailure("could not generate hash from file".to_string())
        })
    }
}

impl Display for SubmissionHash {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.0)
    }
}
