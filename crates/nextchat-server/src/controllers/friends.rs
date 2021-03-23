//! NextChat Server friends controller module.
//!
//! This module contains the routes of the `/friends` path.
//!
//! # Routes
//! `/friends/:user_one/:user_two` -> are_friends
//!
//! See `/src/services/friends.rs` for more information about the routes handlers.

use nextchat_database::{Client, Uuid};
use warp::{filters::BoxedFilter, Filter, Rejection, Reply};

use super::with_client;

/// The prefix of all routes of this module.
fn prefix() -> BoxedFilter<()> {
    warp::path("friends").boxed()
}

/// `/friends/:user_one/:user_two` route declaration.
fn are_friends(client: &Client) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::get()
        .and(prefix())
        .and(warp::path!(Uuid / Uuid))
        .and(with_client(client.clone()))
        .and_then(crate::services::friends::are_friends_handler)
}

/// Combine all `/friends` routes to export.
pub fn routes(client: &Client) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    are_friends(client)
}
