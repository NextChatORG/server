mod users;
mod websockets;

use sqlx::PgPool;
use std::convert::Infallible;
use warp::{filters::BoxedFilter, Filter, Rejection, Reply};
use websockets::{Storage, StorageType};

pub fn v1_path_prefix() -> BoxedFilter<()> {
    warp::path("v1").boxed()
}

pub fn with_storage(
    storage: StorageType,
) -> impl Filter<Extract = (StorageType,), Error = Infallible> + Clone {
    warp::any().map(move || storage.clone())
}

pub fn routes(client: &PgPool) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let storage: StorageType = Storage::default();

    users::routes(client).or(websockets::routes(client, &storage))
}
