use std::fmt::{Display, Formatter};
#[derive(Clone, Debug)]
pub struct SubmissionScore(u16, u16);

impl SubmissionScore {
    pub fn new(passed: u16, total: u16) -> Self {
        Self(passed, total)
    }

    pub fn as_f32(&self) -> f32 {
        (self.0 as f32) / (self.1 as f32)
    }
}

impl Into<f32> for SubmissionScore {
    fn into(self) -> f32 {
        self.as_f32()
    }
}

impl Display for SubmissionScore {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}/{}", self.0, self.1)
    }
}
