/*!
Module that provides the type for the submission scored
 */

use serde::Serialize;
use std::{
    borrow::Cow,
    fmt::{Display, Formatter},
};

#[derive(Clone, Debug)]
pub enum Error {
    ParseIntError(std::num::ParseIntError),
    MalformedString(String),
}

impl std::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Error::ParseIntError(parse_int_error) => parse_int_error.to_string(),
            Error::MalformedString(s) => format!("malformed string: {}", s),
        };
        write!(f, "{}", value)
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(value: std::num::ParseIntError) -> Self {
        Error::ParseIntError(value)
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct SubmissionScore(u16, u16);

impl SubmissionScore {
    pub fn new(passed: u16, total: u16) -> Self {
        Self(passed, total)
    }

    pub fn as_f32(&self) -> f32 {
        (self.0 as f32) / (self.1 as f32)
    }

    pub fn as_f64(&self) -> f64 {
        (self.0 as f64) / (self.1 as f64)
    }
}

impl PartialEq for SubmissionScore {
    fn eq(&self, other: &Self) -> bool {
        self.as_f64() == other.as_f64()
    }
}

impl TryFrom<&str> for SubmissionScore {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let splitted: Vec<&str> = value.trim().split("/").collect();
        if splitted.len() < 2 {
            return Err(Error::MalformedString(format!(
                "{} is not in the form <success>/<total>",
                value
            )));
        }
        let first = splitted[0].parse::<u16>()?;
        let second = splitted[1].parse::<u16>()?;
        Ok(Self::new(first, second))
    }
}

impl TryFrom<Cow<'_, str>> for SubmissionScore {
    type Error = Error;

    fn try_from(value: Cow<'_, str>) -> Result<Self, Self::Error> {
        let str_ref = value.as_ref();
        Self::try_from(str_ref)
    }
}

impl TryFrom<&Cow<'_, str>> for SubmissionScore {
    type Error = Error;

    fn try_from(value: &Cow<'_, str>) -> Result<Self, Self::Error> {
        let str_ref = value.as_ref();
        Self::try_from(str_ref)
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_try_from_success() {
        let tests = vec![
            ("10/11", SubmissionScore(10, 11)),
            ("11/11", SubmissionScore(11, 11)),
            ("10/11 ", SubmissionScore(10, 11)),
            ("10/11\n", SubmissionScore(10, 11)),
            ("0/1 ", SubmissionScore(0, 1)),
        ];

        for (input, expected) in tests {
            let result = SubmissionScore::try_from(input);
            match result {
                Err(e) => assert!(false, "failed to parse {}", input),
                Ok(s) => assert_eq!(s, expected),
            }
        }
    }
}
