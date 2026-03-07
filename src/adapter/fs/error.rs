use std::fmt::{Display, Formatter};

pub enum FileSystemError {
    UnresolvedPath,
    Failure(String),
}

impl Display for FileSystemError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        let value = match self {
            FileSystemError::UnresolvedPath => "could not resolve path,",
            FileSystemError::Failure(s) => s,
        };
        write!(f, "{}", value)
    }
}
