mod friends;
mod users;
mod version_checker;
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

pub struct ResponseBody<T: Serialize> {
    status_code: u16,
    json: T,
}

impl<T: Serialize> ResponseBody<T> {
    pub fn new(status_code: u16, json: T) -> Self {
        Self { status_code, json }
    }

    pub fn new_success(json: T) -> Self {
        Self {
            status_code: 200,
            json,
        }
    }

    pub fn to_reply(&self) -> reply::WithStatus<warp::reply::Json> {
        reply::with_status(
            reply::json(&self.json),
            StatusCode::from_u16(self.status_code).expect("Cannot parse the status code."),
        )
    }
}

pub fn routes(client: &PgPool) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let storage: StorageType = Storage::default();

    version_checker::routes()
        .or(users::routes(client))
        .or(friends::routes(client))
        .or(websockets::routes(client, &storage))
}
