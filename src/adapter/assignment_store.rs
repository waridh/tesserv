/*!
Module that implements the collection
 */

use crate::types::{assignment_config::AssignmentConfig, assignment_id::AssignmentId};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;

type AssignmentPair = (AssignmentId, AssignmentConfig);
/**
Struct that provides access to assignment task configuration based on different
assignment id.
 */
pub struct AssignmentStore {
    assignments: Arc<RwLock<HashMap<AssignmentId, AssignmentConfig>>>,
}

impl AssignmentStore {
    pub fn new() -> Self {
        Self {
            assignments: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn contains_key(&self, key: &AssignmentId) -> bool {
        let locked = self.assignments.read().await;
        locked.contains_key(key)
    }
    pub async fn get(&self, key: &AssignmentId) -> Option<AssignmentConfig> {
        // NOTE: The cloning here could be expensive
        self.assignments.read().await.get(key).cloned()
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
