
pub mod connection;
pub mod models;
pub mod routes;
pub mod schema;
pub mod error;

use std::time;

use axum::{
    routing::get,
    response::Response,
    body::Body,
    http::{HeaderMap, Request},
    Router,
};
use tower_http::classify::ServerErrorsFailureClass;
use tower_http::trace::TraceLayer;
use tracing::Span;

use crate::routes::{root, products};

pub async fn app() -> Router {

    let products_subroutes = Router::new()
        .route("/:id", get(products::get_product));

    let api_subroutes = Router::new()
        .route("/info", get(root::info))
        .route("/authors", get(root::authors))
        .route("/version", get(root::version))
        .nest("/products", products_subroutes);

    Router::new()
        .nest("/api", api_subroutes)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|request: &Request<Body>| {
                    tracing::debug_span!(
                        "http-request",
                        method=?request.method(),
                        uri=?request.uri().path(),
                        version=?request.version(),
                        user_agent=?request.headers().get("user-agent")
                    )
                })
                .on_request(|_request: &Request<Body>, _span: &Span| {
                    tracing::debug!("Request")
                    // tracing::debug_span!("started", method=request.method(), uri=request.uri().path())
                })
                .on_response(
                    |response: &Response, latency: time::Duration, _span: &Span| {
                        tracing::debug!("Response Status='{}' in {:?}", response.status(),latency)
                    },
                )
                .on_eos(
                    |_trailers: Option<&HeaderMap>,
                     stream_duration: time::Duration,
                     _span: &Span| {
                        tracing::debug!("stream closed after {:?}", stream_duration)
                    },
                )
                .on_failure(
                    |_error: ServerErrorsFailureClass, _latency: time::Duration, _span: &Span| {
                        tracing::debug!("Something went wrong")
                    },
                ),
        )
}