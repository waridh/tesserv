/*!
Module that contains the definition of the job ID type
 */

use std::fmt::{Display, Formatter};
use uuid::Uuid;

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct JobId(Uuid);

impl JobId {
    pub fn new() -> Self {
        JobId(Uuid::new_v4())
    }
}

impl Display for JobId {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.0)
    }
}

impl From<Uuid> for JobId {
    fn from(v: Uuid) -> Self {
        JobId(v)
    }
}
impl From<&Uuid> for JobId {
    fn from(v: &Uuid) -> Self {
        JobId(v.clone())
    }
}

impl From<&dyn AsRef<Uuid>> for JobId {
    fn from(v: &dyn AsRef<Uuid>) -> Self {
        JobId::from(v.as_ref())
    }
}

impl AsRef<Uuid> for JobId {
    fn as_ref(&self) -> &Uuid {
        &self.0
    }
}
