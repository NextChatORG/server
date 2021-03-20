//! NextChat Server response module.

use serde::Serialize;
use warp::{http::StatusCode, reply};

pub struct Response<T: Serialize> {
    status_code: u16,
    data: T,
}

impl<T: Serialize> Response<T> {
    /// Create a new response with a custom status code.
    pub fn new(status_code: u16, data: T) -> Self {
        Self { status_code, data }
    }

    /// Create a new response with a 200 status code.
    pub fn new_success(data: T) -> Self {
        Self {
            status_code: 200,
            data,
        }
    }

    /// Convert the response data to a warp reply with a status code.
    pub fn to_reply(&self) -> reply::WithStatus<reply::Json> {
        reply::with_status(
            reply::json(&self.data),
            StatusCode::from_u16(self.status_code).expect("Cannot parse the status code."),
        )
    }
}

#[derive(Serialize)]
pub struct Error {
    message: String,
}

impl Error {
    /// Crate a new error message from a `String`.
    pub fn new(message: String) -> Self {
        Self { message }
    }

    /// Create a new error message from a `&str`.
    pub fn from_str(message: &str) -> Self {
        Self {
            message: String::from(message),
        }
    }

    /// Convert the Error object to a Response object with a status code.
    pub fn to_response(&self, status_code: u16) -> Response<&Error> {
        Response::new(status_code, self)
    }
}
