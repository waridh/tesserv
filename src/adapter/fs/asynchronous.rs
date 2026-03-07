/*!
Module that provides the asynchronous version of the file system helper
 */
use super::error::FileSystemError;
use std::path::Path;
use tokio::fs::create_dir_all;

/**
Helper function that takes in a path, and ensures that the parent path exists.
If the parent path does not exist, then this function shall construct it.
 */
pub async fn ensure_parent<P: AsRef<Path>>(path: P) -> Result<(), FileSystemError> {
    let path_ref = path.as_ref();
    let path_parent = path_ref.parent();
    if let Some(p) = path_parent {
        if p.exists() {
            Ok(())
        } else {
            match create_dir_all(p).await {
                Ok(()) => Ok(()),
                Err(e) => Err(FileSystemError::Failure(e.to_string())),
            }
        }
    } else {
        Err(FileSystemError::UnresolvedPath)
    }
}
