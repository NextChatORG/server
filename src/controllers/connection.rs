//! NextChat Server connection controller module.

use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;
use warp::{ws::Ws, Filter, Rejection, Reply};

use crate::core::{
    database::with_client,
    storage::{with_storage, StorageType},
};

pub fn routes(
    client: &PgPool,
    storage: &StorageType,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    #[derive(Deserialize)]
    struct Query {
        pub user_id: Uuid,
    }

    warp::ws()
        .and(warp::path("connection"))
        .and(warp::query::<Query>())
        .and(with_client(client.clone()))
        .and(with_storage(storage.clone()))
        .map(
            |websocket: Ws, query: Query, client: PgPool, storage: StorageType| {
                websocket.on_upgrade(move |socket| {
                    crate::services::connection::on_new_connection(
                        socket,
                        query.user_id,
                        client,
                        storage,
                    )
                })
            },
        )
}
