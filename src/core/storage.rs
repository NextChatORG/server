//! NextChat Server storage module.
//!
//! This module contains the storage structure for manage the user connections.

use std::{collections::HashMap, convert::Infallible, sync::Arc};

use tokio::sync::{mpsc::error::SendError, RwLock};
use uuid::Uuid;
use warp::{ws::Message, Filter};

use crate::models::connection::Connection;

use super::versions::AppVersions;

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
    /// let storage: StorageType = Storage::default();
    /// ```
    pub fn default() -> StorageType {
        Arc::new(RwLock::new(Self {
            connections: HashMap::new(),
            versions: AppVersions::default(),
        }))
    }

    /// Add a new connection to the connections list.
    pub fn add_connection(&mut self, user_id: &Uuid, connection: Connection) {
        self.connections.insert(user_id.clone(), connection);
    }

    /// Remove a connection by the user id from the connections list.
    pub fn remove_connection(&mut self, user_id: &Uuid) {
        self.connections.remove(user_id);
    }

    /// Send a text message using the websocket connection to all connections.
    pub fn send_text_message_to_all_connections(
        &self,
        message: &str,
    ) -> Result<(), SendError<Result<Message, warp::Error>>> {
        for (_, conn) in self.connections.iter() {
            conn.send_text_message(message)?;
        }

        Ok(())
    }

    /// Get the app versions object.
    pub fn get_versions(&self) -> AppVersions {
        self.versions.clone()
    }
}

/// This function helps to add a copy of the storage to a warp path.
///
/// # Example
/// ```rust
/// let storage: StorageType = Storage::default();
///
/// async fn handler(storage: StorageType) {
///     let mut storage = storage.write().await;
///     storage.connections.insert(user_id, connection);
/// }
///
/// let route = warp::path("testing")
///     .and(with_storage(storage))
///     .and_then(handler);
/// ```
pub fn with_storage(
    storage: StorageType,
) -> impl Filter<Extract = (StorageType,), Error = Infallible> + Clone {
    warp::any().map(move || storage.clone())
}
