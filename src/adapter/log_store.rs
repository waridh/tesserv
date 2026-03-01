/*! module that provides async access to a log store.
 */

use super::file_system::ensure_parent;
use std::{env, fs::File, io::Write, path::Path, path::PathBuf, sync::Arc};
use tokio::sync::RwLock;

fn application_data_home() -> Option<PathBuf> {
    env::var("XDG_DATA_HOME")
        .and_then(|p| Ok(PathBuf::from(p)))
        .ok()
        .or_else(|| env::home_dir())
        .or_else(|| env::var("HOME").map(|v| PathBuf::from(v)).ok())
        .map(|p| p.join("tesserv"))
}

/** Function that tries to resolve the default path for the log filestore.
 */
fn default_path_resolution() -> Result<PathBuf, LogStoreError> {
    // TODO: Current architecture is to panic when there is no home dir
    // find a sane default later
    // TODO: Make the application name be grabbed from some input parameter
    let target_path = application_data_home();
    let default_loc_realized = if let Some(x) = target_path {
        x
    } else {
        return Err(LogStoreError::LogStoreInitError(
            "failed to find a default location for data store".to_string(),
        ));
    };
    if let Err(e) = ensure_parent(&default_loc_realized) {
        return Err(LogStoreError::LogStoreInitError(e.to_string()));
    }
    Ok(default_loc_realized)
}

pub enum LogStoreError {
    LogStoreInitError(String),
}

#[derive(Clone, Debug)]
pub struct LogStore {
    store_loc: Arc<RwLock<File>>,
}

impl LogStore {
    pub fn try_new() -> Result<Self, LogStoreError> {
        let default_path = default_path_resolution()?;
        LogStore::try_from(default_path.as_path())
    }
    async fn write_all(&mut self, v: &[u8]) -> Result<(), ()> {
        let mut file = self.store_loc.write().await;
        file.lock();
        let res = file.write_all(v);
        file.unlock();
        if let Ok(_) = res { Ok(()) } else { Err(()) }
    }
}

/**
 */
impl TryFrom<&Path> for LogStore {
    type Error = LogStoreError;
    fn try_from(path: &Path) -> Result<Self, Self::Error> {
        let file_res = if path.exists() {
            File::options()
                .read(true)
                .append(true)
                .create(true)
                .open(path)
        } else {
            File::options()
                .read(true)
                .write(true)
                .create(true)
                .open(path)
        };
        let file = if let Ok(x) = file_res {
            x
        } else {
            return Err(LogStoreError::LogStoreInitError(
                "could not create file".to_string(),
            ));
        };
        let store_loc = Arc::new(RwLock::new(file));
        Ok(LogStore { store_loc })
    }
}
