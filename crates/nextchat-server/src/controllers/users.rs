//! NextChat Server users controller module.
//!
//! This module contains the routes of the `/users` path.
//!
//! # Routes
//! `/users/all`                        -> get_all
//! `/users/search/:text_to_search`     -> search
//! `/users/find?id={user_id}`          -> find
//! `/users/find?username={username}`   -> find
//! `/users/signup`                     -> signup
//! `/users/signin`                     -> signin
//!
//! See `/src/services/users.rs` for more information about the routes handlers.

use nextchat_database::{models::users::*, Client};
use warp::{filters::BoxedFilter, Filter, Rejection, Reply};

use super::with_client;

/// The prefix of all routes of this module.
fn prefix() -> BoxedFilter<()> {
    warp::path("users").boxed()
}

/// `/users/all` route declaration.
///
/// # Query
/// - `?take={number}`
/// - `?skip={number}`
fn get_all(client: &Client) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::get()
        .and(prefix())
        .and(warp::path("all"))
        .and(warp::query::<GetAllQuery>())
        .and(with_client(client.clone()))
        .and_then(crate::services::users::get_all_handler)
}

/// `/users/search/:text_to_search` route declaration.
///
/// # Query
/// - `?take={number}`
/// - `?skip={number}`
fn search(client: &Client) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::get()
        .and(prefix())
        .and(warp::path!("search" / String))
        .and(warp::query::<SearchQuery>())
        .and(with_client(client.clone()))
        .and_then(crate::services::users::search_handler)
}

/// `/users/find` route declaration.
///
/// # Query
/// - `?id={user_id}`
/// - `?username={username}`
fn find(client: &Client) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::get()
        .and(prefix())
        .and(warp::path("find"))
        .and(warp::query::<FindQuery>())
        .and(with_client(client.clone()))
        .and_then(crate::services::users::find_handler)
}

/// `/users/signup` route declaration.
///
/// # Body
/// ```json
/// {
///     "username": "NextChat",
///     "password": "1234"
/// }
/// ```
fn signup(client: &Client) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::post()
        .and(prefix())
        .and(warp::path("signup"))
        .and(warp::body::json::<SignUpAndSignInBody>())
        .and(with_client(client.clone()))
        .and_then(crate::services::users::signup_handler)
}

/// `/users/signin` route declaration.
///
/// # Body
/// ```json
/// {
///     "username": "NextChat",
///     "password": "1234"
/// }
/// ```
fn signin(client: &Client) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::post()
        .and(prefix())
        .and(warp::path("signin"))
        .and(warp::body::json::<SignUpAndSignInBody>())
        .and(with_client(client.clone()))
        .and_then(crate::services::users::signin_handler)
}

/// Combine all `/users` routes to export.
pub fn routes(client: &Client) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    get_all(client)
        .or(search(client))
        .or(find(client))
        .or(signup(client))
        .or(signin(client))
}
