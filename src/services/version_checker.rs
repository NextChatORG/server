//! NextChat Server version checker service module.
//!
//! This module contains the handlers of the version checker controller routes.
//!
//! `/version_checker/:version` -> index_handler

use std::convert::Infallible;

use semver::Version;
use serde::Serialize;
use warp::Reply;

use crate::core::{
    response::{Error, Response},
    storage::StorageType,
};

/// `/version_checker/:version` handler.
///
/// # Response
/// ```json
/// {
///     "is_deprecated": false,
///     "last_version": null
/// }
/// ```
///
/// ## Errors
/// 1. Cannot parse the version.
/// 2. If the version is higher than the latest version:
/// ```json
/// {
///     "message": "The lastest version is {version}."
/// }
/// ```
/// 3. If the version does not exist.
/// ```json
/// {
///     "message": "The version '{version}' does not exist."
/// }
/// ```
pub async fn index_handler(
    version: String,
    storage: StorageType,
) -> Result<impl Reply, Infallible> {
    let storage = storage.read().await;

    // Parse the version string to a semver version.
    let version = match Version::parse(&version) {
        Ok(version) => version,
        Err(_) => {
            return Ok(Error::from_str("Cannot parse the version.")
                .to_response(400)
                .to_reply());
        }
    };

    let last_version = storage.get_versions().get_last_version();

    // Check if the version param is higher than the last app version.
    if version > last_version.get_version() {
        return Ok(Error::new(format!(
            "The lastest version is {}.",
            last_version.get_version()
        ))
        .to_response(400)
        .to_reply());
    }

    // Check if the version param exists.
    if !storage.get_versions().exists(&version) {
        return Ok(Error::new(format!(
            "The version '{}' does not exist.",
            version.to_string()
        ))
        .to_response(400)
        .to_reply());
    }

    #[derive(Serialize)]
    struct ResponseData {
        pub is_deprecated: bool,
        pub last_version: Option<String>,
    }

    if !storage.get_versions().is_available(&version) {
        return Ok(Response::new(
            400,
            ResponseData {
                is_deprecated: true,
                last_version: Some(
                    storage
                        .get_versions()
                        .get_last_version()
                        .get_version()
                        .to_string(),
                ),
            },
        )
        .to_reply());
    }

    Ok(Response::new_success(ResponseData {
        is_deprecated: false,
        last_version: None,
    })
    .to_reply())
}
