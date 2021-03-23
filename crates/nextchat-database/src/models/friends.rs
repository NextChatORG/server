//! NextChat Database friends models module.
//!
//! This module contains the FriendState enum type and the FriendModel
//! structure for database queries.

use chrono::NaiveDateTime;
use sqlx::{postgres::PgRow, Row};
use uuid::Uuid;

#[derive(sqlx::Type, Clone)]
#[sqlx(type_name = "friends_state", rename_all = "lowercase")]
pub enum FriendState {
    Requested,
    Approved,
}

impl FriendState {
    /// Check if the friend state is requested.
    pub fn is_requested(&self) -> bool {
        match self {
            FriendState::Requested => true,
            _ => false,
        }
    }

    /// Check if the friend state is approved.
    pub fn is_approved(&self) -> bool {
        match self {
            FriendState::Approved => true,
            _ => false,
        }
    }
}

#[derive(Clone)]
pub struct FriendModel {
    transmitter: Uuid,
    receiver: Uuid,
    state: FriendState,
    since: NaiveDateTime,
}

impl FriendModel {
    /// Parse a SQLx row to a FriendModel struct.
    pub fn from_row(row: &PgRow) -> Self {
        Self {
            transmitter: row
                .try_get("transmitter")
                .expect("Cannot parse the friend transmitter id."),
            receiver: row
                .try_get("receiver")
                .expect("Cannot parse the friend receiver id."),
            state: row
                .try_get("state")
                .expect("Cannot parse the friend state."),
            since: row
                .try_get("since")
                .expect("Cannot parse the friend since timestamp."),
        }
    }

    /// Get the friend state (Approved or requested).
    pub fn get_state(&self) -> FriendState {
        self.state.clone()
    }

    /// Get the since timestamp.
    pub fn get_since(&self) -> NaiveDateTime {
        self.since
    }
}
