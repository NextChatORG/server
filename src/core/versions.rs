//! NextChat Server versions module.

mod all_versions;

use std::fmt;

use semver::Version;

#[derive(Clone)]
pub struct AppVersion {
    version: Version,
    deprecated: bool,
}

impl AppVersion {
    /// Create a new app version.
    pub fn new(version: &str, deprecated: bool) -> Self {
        Self {
            version: Version::parse(version).expect("Cannot parse the version."),
            deprecated,
        }
    }

    /// Get the semver version object.
    pub fn get_version(&self) -> Version {
        self.version.clone()
    }

    /// Check if the current version is deprecated.
    pub fn is_deprecated(&self) -> bool {
        self.deprecated
    }
}

impl fmt::Debug for AppVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Version: {}", self.version)
    }
}

#[derive(Clone)]
pub struct AppVersions {
    versions: Vec<AppVersion>,
    last_version: AppVersion,
}

impl AppVersions {
    /// Get the default versions.
    ///
    /// See `/src/core/versions/all_versions.rs` for more information.
    pub fn default() -> Self {
        let versions = all_versions::get();

        let mut last_version: AppVersion = AppVersion::new("0.0.0", false);
        for version in versions.iter() {
            if version.get_version() > last_version.get_version() {
                last_version = version.clone();
                continue;
            }
        }

        Self {
            versions,
            last_version,
        }
    }

    /// Get the last version.
    pub fn get_last_version(&self) -> AppVersion {
        self.last_version.clone()
    }

    /// Check if the version exists.
    pub fn exists(&self, version: &Version) -> bool {
        for v in self.versions.iter() {
            if &v.get_version() == version {
                return true;
            }
        }

        false
    }

    /// Check if a version exists and is not deprecated.
    pub fn is_available(&self, version: &Version) -> bool {
        for v in self.versions.iter() {
            if &v.get_version() == version {
                if !v.is_deprecated() {
                    return true;
                }

                break;
            }
        }

        false
    }
}
