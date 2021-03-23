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
/// use std::{convert::Infallible, env};
///
/// use nextchat_database::{Client, get_client, query};
/// use nextchat_server::{Response, with_client};
/// use serde::Serialize;
/// use warp::{Filter, Reply};
///
/// #[tokio::main]
/// async fn main() {
///     env::set_var("DATABASE_URL", "postgres://postgres:password@localhost/nextchat");
///
///     let database_connection: Client = match get_client().await {
///         Ok(db) => db,
///         Err(e) => {
///             eprintln!("Database Error: {:?}", e);
///             return;
///         },
///     };
///
///     async fn handler(client: Client) -> Result<impl Reply, Infallible> {
///         #[derive(Serialize)]
///         struct ResponseData {
///             updated: bool,
///         }
///
///         let affected = query("UPDATE FROM users SET online = false WHERE username = 'NextChat'")
///             .execute(&client)
///             .await
///             .expect("Cannot execute the query.")
///             .rows_affected();
///
///         Ok(Response::new_success(ResponseData { updated: affected != 0 }).to_reply())
///     }
///
///     let route = warp::get()
///         .and(warp::path("testing"))
///         .and(with_client(database_connection))
///         .and_then(handler);
/// }
/// ```
pub fn with_client(client: Client) -> impl Filter<Extract = (Client,), Error = Infallible> + Clone {
    warp::any().map(move || client.clone())
}

/// This function helps to add a copy of the storage to a warp path.
///
/// # Example
/// ```rust
/// use std::convert::Infallible;
///
/// use nextchat_communication::{Storage, StorageType};
/// use nextchat_server::{Response, with_storage};
/// use serde::Serialize;
/// use warp::{Filter, Reply};
///
/// fn main() {
///     let storage: StorageType = Storage::default();
///
///     async fn handler(storage: StorageType) -> Result<impl Reply, Infallible> {
///         let storage = storage.read().await;
///         let last_version = storage.get_versions().get_last_version();
///
///         #[derive(Serialize)]
///         struct ResponseData {
///             pub version: String,
///         }
///
///         Ok(Response::new_success(ResponseData {
///             version: last_version.get_version().to_string(),
///         }).to_reply())
///     }
///
///     let route = warp::get()
///         .and(warp::path("testing"))
///         .and(with_storage(storage))
///         .and_then(handler);
/// }
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
