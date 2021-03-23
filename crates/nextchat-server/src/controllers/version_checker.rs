//! NextChat Server version checker controller module.
//!
//! This module contains the routes of the `/version_checker` path.
//!
//! # Routes
//! `/version_checker/:version`
//!
//! See `/src/services/version_checker.rs` for more information about the routes handlers.

use warp::{Filter, Rejection, Reply};

use nextchat_communication::StorageType;

use super::with_storage;

/// Combine all `/version_checker` routes to export.
pub fn routes(
    storage: &StorageType,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::patch()
        .and(warp::path!("version_checker" / String))
        .and(with_storage(storage.clone()))
        .and_then(crate::services::version_checker::index_handler)
}
