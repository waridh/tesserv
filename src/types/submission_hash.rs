use crate::hasher::hash_file;
use serde::Serialize;
use std::{
    fmt::{Display, Formatter},
    path::Path,
};

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize)]
pub struct SubmissionHash(String);

#[derive(Clone, Debug)]
pub enum SubmissionHashError {
    FileFailure(String),
}

impl Display for SubmissionHashError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            SubmissionHashError::FileFailure(s) => format!("submission hash failure {}", s),
        };
        write!(f, "{value}")
    }
}

impl std::error::Error for SubmissionHashError {}

impl TryFrom<&Path> for SubmissionHash {
    type Error = SubmissionHashError;

    fn try_from(value: &Path) -> Result<Self, Self::Error> {
        hash_file(value).map(|s| Self(s)).map_err(|_| {
            SubmissionHashError::FileFailure("could not generate hash from file".to_string())
        })
    }
}

impl Display for SubmissionHash {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.0)
    }
}
