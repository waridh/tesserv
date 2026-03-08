/*!
Module that implements the collection
 */

use super::tesserv_config;
use crate::types::{assignment_config::AssignmentConfig, assignment_id::AssignmentId};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;

type AssignmentPair = (AssignmentId, AssignmentConfig);
/**
Struct that provides access to assignment task configuration based on different
assignment id.
 */
#[derive(Clone)]
pub struct AssignmentStore {
    assignments: Arc<RwLock<HashMap<AssignmentId, AssignmentConfig>>>,
}

impl AssignmentStore {
    pub fn new() -> Self {
        Self {
            assignments: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    #[allow(dead_code)]
    pub async fn contains_key<K>(&self, key: K) -> bool
    where
        K: Into<AssignmentId>,
    {
        let locked = self.assignments.read().await;
        locked.contains_key(&key.into())
    }
    pub async fn get<K>(&self, key: K) -> Option<AssignmentConfig>
    where
        K: Into<AssignmentId>,
    {
        // NOTE: The cloning here could be expensive
        self.assignments.read().await.get(&key.into()).cloned()
    }
}

impl Default for AssignmentStore {
    fn default() -> Self {
        Self::new()
    }
}

impl FromIterator<AssignmentPair> for AssignmentStore {
    fn from_iter<T: IntoIterator<Item = (AssignmentId, AssignmentConfig)>>(iter: T) -> Self {
        let mut store = HashMap::new();
        for (id, config) in iter {
            store.insert(id, config);
        }
        let assignments = Arc::new(RwLock::new(store));
        Self { assignments }
    }
}

impl From<&tesserv_config::TesservConfig> for AssignmentStore {
    fn from(value: &tesserv_config::TesservConfig) -> Self {
        let assignments = value.assignments().iter().map(|e| e.config_pair());
        Self::from_iter(assignments)
    }
}
