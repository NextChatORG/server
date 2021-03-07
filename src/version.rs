use semver::Version as SVersion;

#[derive(Clone, Debug)]
pub struct Version {
    general_version: SVersion,
    required_update: bool,
}

impl Version {
    pub fn new(general_version: SVersion, required_update: bool) -> Self {
        Self {
            general_version,
            required_update,
        }
    }

    pub fn get_version(&self) -> SVersion {
        self.general_version.clone()
    }

    pub fn required_update(&self) -> bool {
        self.required_update
    }
}

pub fn get_versions() -> Vec<Version> {
    [Version::new(
        SVersion::parse("1.0.0-alpha1").unwrap(),
        false,
    )]
    .to_vec()
}
