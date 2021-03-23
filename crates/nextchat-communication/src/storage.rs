//! NextChat Communication storage module.
//!
//! This module contains the storage structure for manage the user connections.

use std::{collections::HashMap, sync::Arc};

use nextchat_database::Uuid;
use nextchat_utils::AppVersions;
use tokio::sync::RwLock;

use crate::Connection;

pub struct Storage {
    connections: HashMap<Uuid, Connection>,
    versions: AppVersions,
}

pub type StorageType = Arc<RwLock<Storage>>;

impl Storage {
    /// Generate a new empty storage.
    ///
    /// # Example
    /// ```rust
    /// use nextchat_communication::{Storage, StorageType};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let storage: StorageType = Storage::default();
    ///     let storage = storage.read().await;
    ///
    ///     let last_version = storage.get_versions().get_last_version();
    ///     assert!(!last_version.is_deprecated());
    /// }
    /// ```
    pub fn default() -> StorageType {
        Arc::new(RwLock::new(Self {
            connections: HashMap::new(),
            versions: AppVersions::default(),
        }))
    }

    /// Add a new connection to the connections list.
    pub fn add_connection(&mut self, user_id: &Uuid, connection: &Connection) {
        self.connections.insert(user_id.clone(), connection.clone());
    }

    /// Remove a connection by the user id from the connections list.
    pub fn remove_connection(&mut self, user_id: &Uuid) {
        self.connections.remove(user_id);
    }

    /// Get the app versions object.
    pub fn get_versions(&self) -> AppVersions {
        self.versions.clone()
    }
}
