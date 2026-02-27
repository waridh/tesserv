/*! module that provides async access to a log store.
 */

use std::{fs::File, path::Path};

struct LogStore {}

struct LogStoreError {}

impl LogStore {
    pub fn new() -> Self {
        LogStore {}
    }
}

impl Default for LogStore {
    fn default() -> Self {
        Self::new()
    }
}

impl TryFrom<&Path> for LogStore {
    type Error = LogStoreError;
    fn try_from(path: &Path) -> Result<Self, Self::Error> {
        let file_res = if (path.exists()) {
            File::create(path.as_os_str())
        } else {
            File::create(path.as_os_str())
        };
        let file = if let Ok(x) = file_res {
            x
        } else {
            return Err(LogStoreError {});
        };
        Err(LogStoreError {})
    }
}
