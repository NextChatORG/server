mod events;
mod storage;

pub use storage::{Storage, StorageType};

use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;
use warp::{ws::Ws, Filter, Rejection, Reply};

pub fn routes(
    client: &PgPool,
    storage: &StorageType,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    #[derive(Deserialize)]
    struct WebSocketQuery {
        pub user_id: Uuid,
    }

    warp::ws()
        .and(warp::path("ws"))
        .and(warp::query::<WebSocketQuery>())
        .and(super::with_client(client.clone()))
        .and(super::with_storage(storage.clone()))
        .map(
            |ws: Ws, query: WebSocketQuery, client: PgPool, storage: StorageType| {
                ws.on_upgrade(move |socket| {
                    events::on_user_connected(socket, query.user_id, client, storage)
                })
            },
        )
}
