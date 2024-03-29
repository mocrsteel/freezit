//! Freezit API. 
#![warn(missing_docs)]

pub mod core;
pub mod models;
pub mod routes;
#[allow(missing_docs)]
pub mod schema;

use std::time::Duration;

use axum::{
    routing::{get, post, patch, delete},
    response::Response,
    body::Body,
    http::{HeaderMap, Request},
    Router,
};
use tower_http::classify::ServerErrorsFailureClass;
use tower_http::{
    timeout::TimeoutLayer,
    trace::TraceLayer
};
use tracing::Span;

use crate::routes::{root, products, freezers, drawers, storage};

/// Contains application state variables.
#[derive(Clone)]
pub struct AppState {
    db_url: Option<String>,
}

/// App factory with possibility to define non-.env database url.
pub async fn app(db_url: Option<String>) -> Router {
    let state = AppState {
        db_url,
    };

    let products_subroutes = Router::new()
        .route("/", get(products::get_all_products))
        .route("/", patch(products::update_product))
        .route("/create", post(products::create_product))
        .route("/id=:id", get(products::get_product_by_id))
        .route("/id=:id", delete(products::delete_product))
        .route("/name=:name", get(products::get_product_by_name))
        .route("/expiration=:expiration", get(products::get_products_by_expiration));

    let freezer_subroutes = Router::new()
        .route("/", get(freezers::get_all_freezers))
        .route("/", patch(freezers::update_freezer))
        .route("/create", post(freezers::create_freezer))
        .route("/id=:id", get(freezers::get_freezer_by_id))
        .route("/id=:id", delete(freezers::delete_freezer))
        .route("/name=:name", get(freezers::get_freezer_by_name));

    let drawer_subroutes = Router::new()
        .route("/", get(drawers::get_drawers))
        .route("/", post(drawers::create_drawer))
        .route("/", patch(drawers::update_drawer))
        .route("/:id", delete(drawers::delete_drawer));

    let storage_subroutes = Router::new()
        .route("/", get(storage::get_storage))
        .route("/:id", get(storage::get_storage_by_id))
        .route("/", post(storage::create_storage))
        .route("/", patch(storage::update_storage))
        .route("/:id/withdraw", patch(storage::withdraw_storage))
        .route("/:id/re-enter", patch(storage::re_enter_storage))
        .route("/:id", delete(storage::delete_storage));

    let api_subroutes = Router::new()
        .route("/", get(|| async { "API active" }))
        .route("/info", get(root::info))
        .route("/authors", get(root::authors))
        .route("/version", get(root::version))
        .nest("/products", products_subroutes)
        .nest("/freezers", freezer_subroutes)
        .nest("/drawers", drawer_subroutes)
        .nest("/storage", storage_subroutes);

    Router::new()
        .nest("/api", api_subroutes)
        .with_state(state)
        .layer(TimeoutLayer::new(Duration::from_secs(15)))
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
                    |response: &Response, latency: Duration, _span: &Span| {
                        tracing::debug!("Response Status='{}' in {:?}", response.status(),latency)
                    },
                )
                .on_eos(
                    |_trailers: Option<&HeaderMap>,
                     stream_duration: Duration,
                     _span: &Span| {
                        tracing::debug!("stream closed after {:?}", stream_duration)
                    },
                )
                .on_failure(
                    |_error: ServerErrorsFailureClass, _latency: Duration, _span: &Span| {
                        tracing::debug!("Something went wrong")
                    },
                ),
        )
}
