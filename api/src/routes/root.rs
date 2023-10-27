use crate::error::internal_error;

use std::env;

use axum::response::Json;
use hyper::StatusCode;
use serde::{Serialize};
use typeshare::typeshare;

const NAME: &str = env!("CARGO_PKG_NAME");
const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
const VERSION: &str = env!("CARGO_PKG_VERSION");
const VERSION_MAJOR: &str = env!("CARGO_PKG_VERSION_MAJOR");
const VERSION_MINOR: &str = env!("CARGO_PKG_VERSION_MINOR");
const VERSION_PATCH: &str = env!("CARGO_PKG_VERSION_PATCH");
const VERSION_PRE: &str = env!("CARGO_PKG_VERSION_PRE");

#[typeshare]
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Version {
    major: u32,
    minor: u32,
    patch: u32,
    pre: Option<String>,
}

pub async fn info() -> String{
    format!("Welcome to {} v{}", NAME, VERSION)
}

pub async fn version() -> Result<Json<Version>, (StatusCode, String)> {
    let pre = match VERSION_PRE {
        "" => None,
        pre => Some(String::from(pre))
    };

    let version = Version {
        major: VERSION_MAJOR.parse::<u32>().map_err(internal_error)?,
        minor: VERSION_MINOR.parse::<u32>().map_err(internal_error)?,
        patch: VERSION_PATCH.parse::<u32>().map_err(internal_error)?,
        pre,
    };

    Ok(Json(version))
}

pub async fn authors() -> &'static str {
    match AUTHORS {
        "" => "No authors defined",
        authors => authors,
    }
}