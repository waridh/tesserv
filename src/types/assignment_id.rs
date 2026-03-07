use serde::Serialize;
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, Hash, Eq, PartialEq, PartialOrd, Ord, Serialize)]
pub struct AssignmentId(String);

impl AssignmentId {
    pub fn new(name: String) -> Self {
        Self(name)
    }
}

impl From<String> for AssignmentId {
    fn from(name: String) -> Self {
        Self::new(name)
    }
}
impl From<&str> for AssignmentId {
    fn from(name: &str) -> Self {
        Self::new(name.to_string())
    }
}

impl Display for AssignmentId {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.0)
    }
}
