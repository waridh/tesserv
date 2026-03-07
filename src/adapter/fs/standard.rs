use super::error::FileSystemError;
use std::{fs::create_dir_all, path::Path};

/**
Helper function that takes in a path, and ensures that the parent path exists.
If the parent path does not exist, then this function shall construct it.
 */
pub fn ensure_parent<P: AsRef<Path>>(path: P) -> Result<(), FileSystemError> {
    let path_ref = path.as_ref();
    let path_parent = path_ref.parent();
    if let Some(p) = path_parent {
        if p.exists() {
            Ok(())
        } else {
            match create_dir_all(p) {
                Ok(()) => Ok(()),
                Err(e) => Err(FileSystemError::Failure(e.to_string())),
            }
        }
    } else {
        Err(FileSystemError::UnresolvedPath)
    }
}
