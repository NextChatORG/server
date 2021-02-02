pub mod users;

use actix_web_actors::ws;
use actix_web::{error, Error as ActixError, http::StatusCode, HttpRequest, HttpResponse, web};
use crate::connection::Connection;
use derive_more::{Display, Error as DeriveError};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, DeriveError, Deserialize, Display, Serialize)]
#[display(fmt = "Bad request")]
pub struct Error {
    message: String,
}

impl Error {
    pub fn new(message: &str) -> Self {
        Self {
            message: String::from(message),
        }
    }
}

impl error::ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(StatusCode::BAD_REQUEST).json(self)
    }
}

#[derive(Deserialize)]
pub struct WebSocketQuery {
    user_id: Uuid,
}

pub async fn websocket(
    req: HttpRequest,
    stream: web::Payload,
    web::Query(info): web::Query<WebSocketQuery>,
) -> Result<HttpResponse, ActixError> {
    let user_id: Uuid = info.user_id;

    ws::start(
        Connection::new(user_id),
        &req,
        stream,
    )
}
