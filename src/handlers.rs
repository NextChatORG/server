mod users;
mod websockets;

use serde::Serialize;
use sqlx::PgPool;
use std::convert::Infallible;
use warp::{http::StatusCode, reply, Filter, Rejection, Reply};
use websockets::{Storage, StorageType};

pub fn with_client(client: PgPool) -> impl Filter<Extract = (PgPool,), Error = Infallible> + Clone {
    warp::any().map(move || client.clone())
}

pub fn with_storage(
    storage: StorageType,
) -> impl Filter<Extract = (StorageType,), Error = Infallible> + Clone {
    warp::any().map(move || storage.clone())
}

#[derive(Serialize)]
pub struct Error {
    code: i64,
    message: String,
}

impl Error {
    pub fn new(code: i64, message: String) -> Self {
        Self { code, message }
    }

    pub fn new_str(code: i64, message: &str) -> Self {
        Self::new(code, String::from(message))
    }
}

pub enum ResponseBody<T: Serialize> {
    Error(Error),
    Success(T),
}

impl<T: Serialize> ResponseBody<T> {
    pub fn new_error(error: Error) -> Self {
        Self::Error(error)
    }

    pub fn new_success(json: T) -> Self {
        Self::Success(json)
    }

    pub fn to_reply(&self) -> reply::WithStatus<warp::reply::Json> {
        match self {
            Self::Error(error) => reply::with_status(reply::json(error), StatusCode::BAD_REQUEST),
            Self::Success(success) => reply::with_status(reply::json(success), StatusCode::OK),
        }
    }
}

pub fn routes(client: &PgPool) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let storage: StorageType = Storage::default();

    users::routes(client).or(websockets::routes(client, &storage))
}
