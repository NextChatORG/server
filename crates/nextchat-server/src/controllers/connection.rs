//! NextChat Server connection controller module.

use nextchat_communication::StorageType;
use nextchat_database::{Client, Uuid};
use serde::Deserialize;
use warp::{ws::Ws, Filter, Rejection, Reply};

use super::{with_client, with_storage};

pub fn routes(
    client: &Client,
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
            |websocket: Ws, query: Query, client: Client, storage: StorageType| {
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
