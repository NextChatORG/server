//! NextChat Server controllers module.

mod connection;
mod friends;
mod users;
mod version_checker;

use std::convert::Infallible;

use nextchat_communication::{Storage, StorageType};
use nextchat_database::Client;
use warp::{Filter, Rejection, Reply};

/// This function helps to add a copy of the database connection to a warp path.
///
/// # Example
/// ```rust
/// let database_connection = nextchat_database::get_client().unwrap();
///
/// async fn handler(client: Client) { }
///
/// let route = warp::path("testing")
///     .and(with_client(database_connection))
///     .and_then(handler);
/// ```
pub fn with_client(client: Client) -> impl Filter<Extract = (Client,), Error = Infallible> + Clone {
    warp::any().map(move || client.clone())
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

/// Combine all controllers routes.
pub fn routes(client: &Client) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let storage = Storage::default();

    users::routes(client)
        .or(friends::routes(client))
        .or(connection::routes(client, &storage))
        .or(version_checker::routes(&storage))
}
