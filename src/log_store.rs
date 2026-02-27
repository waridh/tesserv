/*! module that provides async access to a log store.
 */

use std::{env, fs::File, path::Path};

pub struct LogStore {
    store_loc: File,
}

struct LogStoreError {}

impl LogStore {
    pub fn new() -> Self {
        // TODO: Current architecture is to panic when there is no home dir
        // find a sane default later
        let user_home = env::home_dir().unwrap_or_else(|| panic!());
        let default_loc = user_home.join(".local");
        LogStore {}
    }
}

impl Default for LogStore {
    fn default() -> Self {
        Self::new()
    }
}

/**
 */
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
