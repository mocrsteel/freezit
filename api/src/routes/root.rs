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

#[cfg(test)]
mod tests {
    use super::*;

    use crate::app;
    use axum::{
        body::Body,
        http::Request
    };
    use hyper::body::Bytes;
    use serde_json::{json, Value};
    use tower::util::ServiceExt;

    #[tokio::test]
    async fn api_info_response() {
        let app = app().await;
        let api_version = env!("CARGO_PKG_VERSION");
        let expected_body = Bytes::from(format!("Welcome to api v{}", api_version));
        let response = app
            .oneshot(Request::builder().uri("/api/info").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        assert_eq!(body, expected_body);
    }

    #[tokio::test]
    async fn api_version_response() {
        let app = app().await;

        let version_major = env!("CARGO_PKG_VERSION_MAJOR").parse::<u32>().unwrap();
        let version_minor = env!("CARGO_PKG_VERSION_MINOR").parse::<u32>().unwrap();
        let version_patch = env!("CARGO_PKG_VERSION_PATCH").parse::<u32>().unwrap();
        let version_pre = env!("CARGO_PKG_VERSION_PRE");

        let expected_json = json!({
            "major": version_major,
            "minor": version_minor,
            "patch": version_patch,
            "pre": version_pre
        });
        let response = app
            .oneshot(Request::builder().uri("/api/version").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let json_response: Value = serde_json::from_slice(&body).unwrap();

        assert_eq!(json_response, expected_json);
    }
}