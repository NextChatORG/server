//! NextChat Database users models module.
//!
//! This module contains the structs for the users routes.
//!
//! `/users/all`    query -> GetAllQuery
//! `/users/search` query -> SearchQuery
//! `/users/find`   query -> FindQuery
//! `/users/signup` body  -> SignUpAndSignInBody
//! `/users/signin` body  -> SignUpAndSignInBody

use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgRow, Row};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct GetAllQuery {
    pub skip: Option<i64>,
    pub take: Option<i64>,
}

#[derive(Deserialize)]
pub struct SearchQuery {
    pub skip: Option<i64>,
    pub take: Option<i64>,
}

#[derive(Deserialize)]
pub struct FindQuery {
    pub id: Option<Uuid>,
    pub username: Option<String>,
}

#[derive(Deserialize)]
pub struct SignUpAndSignInBody {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct UserDataResponse {
    pub id: Uuid,
    pub username: String,
    pub profile_image: String,
}

impl UserDataResponse {
    /// Parse a SQLx row to an UserDataResponse.
    pub fn from_row(row: &PgRow) -> Self {
        Self {
            id: row.try_get("id").expect("Cannot parse the user id."),
            username: row.try_get("username").expect("Cannot parse the username."),
            profile_image: row
                .try_get("profile_image")
                .expect("Cannot parse the user profile image."),
        }
    }
}
