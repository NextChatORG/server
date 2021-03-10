use crate::database::get_now_time;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgRow, Row};
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FriendModel {
    user_one: Uuid,
    user_two: Uuid,
    since: NaiveDateTime,
    is_request: bool,
}

impl FriendModel {
    pub fn new_request(user_one: &Uuid, user_two: &Uuid) -> Self {
        Self {
            user_one: user_one.clone(),
            user_two: user_two.clone(),
            since: get_now_time(),
            is_request: true,
        }
    }

    pub fn from_row(row: &PgRow) -> Self {
        Self {
            user_one: row.try_get("user_one").expect("Cannot parse the user one id."),
            user_two: row.try_get("user_two").expect("Cannot parse the user two id."),
            since: row.try_get("since").expect("Cannot parse the since timestamp."),
            is_request: row.try_get("is_request").expect("Cannot parse the is request row."),
        }
    }

    pub fn is_request(&self) -> bool {
        self.is_request
    }
}
