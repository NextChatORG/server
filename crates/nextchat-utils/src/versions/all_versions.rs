//! NextChat Utils all versions module.
//!
//! This module contains the versions list of the NextChat app.

use super::AppVersion;

/// Get all app versions.
pub fn get() -> Vec<AppVersion> {
    [AppVersion::new("0.1.0-alpha1", false)].to_vec()
}
