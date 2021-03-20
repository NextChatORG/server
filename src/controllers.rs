//! NextChat Server controllers module.

mod connection;
mod friends;
mod users;
mod version_checker;

use sqlx::PgPool;
use warp::{Filter, Rejection, Reply};

use crate::core::storage::Storage;

/// Combine all controllers routes.
pub fn routes(client: &PgPool) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let storage = Storage::default();

    users::routes(client)
        .or(friends::routes(client))
        .or(connection::routes(client, &storage))
        .or(version_checker::routes(&storage))
}
