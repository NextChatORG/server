pub mod users;

use actix_web::{error, http::StatusCode, HttpResponse};
use derive_more::{Display, Error as DeriveError};
use serde::{Deserialize, Serialize};

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
