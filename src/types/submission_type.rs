/*!
Module that defines the different supported submission type
 */

use std::path::Path;

pub enum SubmissionType {
    Tar,
}

pub struct FileType(String);

pub enum Error {
    IOError(String),
    FiletypeError(FileType),
}

impl SubmissionType {
    pub fn extension(&self) -> &str {
        match self {
            SubmissionType::Tar => "tar",
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Self::IOError(s) => format!("IOError: {}", s),
            Self::FiletypeError(t) => format!("the type: {} is not modelled", t.0),
        };
        write!(f, "{}", value)
    }
}

impl TryFrom<&str> for SubmissionType {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let path_ref = Path::new(value);
        Self::try_from(path_ref)
    }
}

impl TryFrom<&Path> for SubmissionType {
    type Error = Error;

    fn try_from(value: &Path) -> Result<Self, Self::Error> {
        let filename = value.file_name();
        match filename {
            None => Err(Error::IOError(format!(
                "could not get file name from {}",
                value.display()
            ))),
            Some(x) => {
                if let Some(y) = x.to_str() {
                    if y.ends_with(".tar") {
                        Ok(SubmissionType::Tar)
                    } else {
                        let splited = y.split_once(".");
                        match splited {
                            None => Err(Error::IOError(format!(
                                "could not determine file type: {}",
                                x.display()
                            ))),
                            Some((_, rest)) => {
                                Err(Error::FiletypeError(FileType(rest.to_string())))
                            }
                        }
                    }
                } else {
                    Err(Error::IOError(format!(
                        "could not convert os str to str: {:?}",
                        x
                    )))
                }
            }
        }
    }
}
