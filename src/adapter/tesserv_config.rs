/*!
Module that provides adapter for reading configuration files
 */

use crate::types::{assignment_config::AssignmentConfig, assignment_id::AssignmentId};
use config;
use serde::Deserialize;
use std::path::{Path, PathBuf};

#[derive(Clone, Debug)]
pub enum Error {
    ParseFailure { file: PathBuf, msg: String },
    PathResolutionFailure(String),
}

impl Error {
    pub fn parse_failure<P>(file: P, msg: String) -> Self
    where
        P: AsRef<Path>,
    {
        Self::ParseFailure {
            file: file.as_ref().to_path_buf(),
            msg,
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            Error::ParseFailure { file, msg } => {
                format!(
                    "failed to parse config file {} with message {}",
                    file.display(),
                    msg
                )
            }
            Error::PathResolutionFailure(s) => format!("failed to resolve the path: {s}"),
        };
        write!(f, "{msg}")
    }
}

impl std::error::Error for Error {}

#[derive(Deserialize, Debug, Clone)]
pub struct AssignmentGroup {
    assignment_id: String,
    marking_scripts: Vec<PathBuf>,
}

impl AssignmentGroup {
    fn assignment_id(&self) -> AssignmentId {
        AssignmentId::from(self.assignment_id.as_str())
    }

    fn assignment_config(&self) -> AssignmentConfig {
        AssignmentConfig::from(self.marking_scripts.as_slice())
    }

    pub fn config_pair(&self) -> (AssignmentId, AssignmentConfig) {
        let first = self.assignment_id();
        let second = self.assignment_config();
        (first, second)
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct TesservConfig {
    assignments: Vec<AssignmentGroup>,
}

impl TesservConfig {
    pub fn assignments<'a>(&'a self) -> &'a [AssignmentGroup] {
        &self.assignments
    }

    pub fn try_from_cmd_line<P>(path: P) -> Result<Self, Error>
    where
        P: AsRef<Path>,
    {
        let path_buf = path
            .as_ref()
            .canonicalize()
            .map_err(|_| Error::PathResolutionFailure(path.as_ref().display().to_string()))?;
        let path_ref = path_buf.as_path();
        let builder = config::Config::builder().add_source(config::File::from(path_ref));
        let mut built: TesservConfig = builder
            .build()
            .or_else(|e| Err(Error::parse_failure(path_ref, e.to_string())))
            .and_then(|v| {
                v.try_deserialize()
                    .map_err(|e| Error::parse_failure(path_ref, e.to_string()))
            })?;

        let config_dir = path_ref
            .parent()
            .ok_or_else(|| Error::PathResolutionFailure(path_ref.display().to_string()))?;

        /* imperative programming here due to simplicity and efficiency */
        for assign in built.assignments.iter_mut() {
            for script in assign.marking_scripts.iter_mut() {
                /* TODO: do the case for absolute paths. In those cases,
                there would be no resolutions required
                 */
                if script.is_relative() {
                    let new_path = config_dir.join(&script);
                    *script = new_path;
                }
                *script = script
                    .canonicalize()
                    .map_err(|_| Error::PathResolutionFailure(script.display().to_string()))?;
            }
        }

        Ok(built)
    }
}
