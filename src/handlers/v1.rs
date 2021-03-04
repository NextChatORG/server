mod users;
mod websockets;

use sqlx::PgPool;
use warp::{filters::BoxedFilter, Filter, Rejection, Reply};

pub fn v1_path_prefix() -> BoxedFilter<()> {
    warp::path("v1").boxed()
}

pub fn routes(client: &PgPool) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    users::routes(client).or(websockets::routes(client))
}
