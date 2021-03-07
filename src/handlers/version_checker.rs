use super::{Error, ResponseBody};
use crate::version::{get_versions, Version};
use semver::Version as SVersion;
use serde::{Deserialize, Serialize};
use std::convert::Infallible;
use warp::{Filter, Rejection, Reply};

pub fn routes() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    #[derive(Deserialize)]
    struct Query {
        pub version: String,
    }

    async fn handler(query: Query) -> Result<impl Reply, Infallible> {
        let versions = get_versions();
        let version = match SVersion::parse(&query.version) {
            Ok(version) => version,
            Err(e) => {
                eprintln!("Semver parse error: {:?}", e);
                return Ok(
                    ResponseBody::new(400, Error::new_str(0, "Cannot parse the version."))
                        .to_reply(),
                );
            }
        };

        let mut version_vec: Option<Version> = None;
        for v in versions.iter() {
            if v.get_version() == version {
                version_vec = Some(v.clone());
                break;
            }
        }

        #[derive(Serialize)]
        struct Response {
            pub required_update: bool,
            pub to: Option<String>,
        }

        let latest_version = &versions[versions.len() - 1];

        if version_vec.is_none() {
            return Ok(ResponseBody::new(
                404,
                Response {
                    required_update: true,
                    to: Some(latest_version.get_version().to_string()),
                },
            )
            .to_reply());
        }

        let required_update = version_vec.unwrap().required_update();
        Ok(ResponseBody::new(
            200,
            Response {
                required_update,
                to: if required_update {
                    Some(latest_version.get_version().to_string())
                } else {
                    None
                },
            },
        )
        .to_reply())
    }

    warp::patch()
        .and(warp::path("version_checker"))
        .and(warp::query::<Query>())
        .and_then(handler)
}
