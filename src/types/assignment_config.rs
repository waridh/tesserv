/*!
Module that implements the struct that represents the verification work
 */
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

#[derive(Clone)]
pub struct AssignmentConfig {
    marking_scripts: Arc<Vec<PathBuf>>,
}

impl AssignmentConfig {
    pub fn new() -> Self {
        Self {
            marking_scripts: Arc::new(vec![]),
        }
    }
}

impl Default for AssignmentConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl From<Vec<PathBuf>> for AssignmentConfig {
    fn from(value: Vec<PathBuf>) -> Self {
        Self {
            marking_scripts: Arc::new(value),
        }
    }
}

impl From<Vec<&Path>> for AssignmentConfig {
    fn from(value: Vec<&Path>) -> Self {
        let owned: Vec<PathBuf> = value.iter().map(|e| e.to_path_buf()).collect();
        Self::from(owned)
    }
}

impl AsRef<[PathBuf]> for AssignmentConfig {
    fn as_ref(&self) -> &[PathBuf] {
        self.marking_scripts.as_slice()
    }
}
