pub mod users;

use crate::{connection::Connection, database::User};
use actix_web::{error, http::StatusCode, web, Error as ActixError, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use derive_more::{Display, Error as DeriveError};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, DeriveError, Deserialize, Display, Serialize)]
#[display(fmt = "Bad request")]
pub struct Error {
    code: i64,
    message: String,
}

impl Error {
    pub fn new(code: i64, message: String) -> Self {
        Self { code, message }
    }

    pub fn new_str(code: i64, message: &str) -> Self {
        Self {
            code,
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
    client: web::Data<PgPool>,
    web::Query(info): web::Query<WebSocketQuery>,
) -> Result<HttpResponse, ActixError> {
    let user_id: Uuid = info.user_id;
    let user: User = User::from_id(client.get_ref(), &user_id, false)
        .await
        .expect("Cannot get websocket user.");

    ws::start(Connection::new(user), &req, stream)
}
