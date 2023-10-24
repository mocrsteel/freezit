// use chrono::prelude::*;
// use api::connection::establish_connection;
// use api::models::{NewStorageItem, Product, Storage};
// use diesel::prelude::*;
// use diesel::result::Error;
// use api::schema::storage::dsl::storage;
use axum::{
    routing::get,
    response::Response,
    body::Body,
    http::{HeaderMap, Request},
    Router
};
// use hyper::{Body, HeaderMap, Request, Response};
use std::time;
use axum::http::HeaderValue;
use tower_http::classify::ServerErrorsFailureClass;
use tower_http::trace::TraceLayer;
use tracing::Span;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "api=debug,tower_http=debug,axum::rejection=trace".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app = Router::new()
        .route("/", get(|| async { "Hello World!" }))
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
        );
    let addr = ([0, 0, 0, 0], 3000).into();
    let server = axum::Server::bind(&addr);
    tracing::debug!("listening on {} at port {}", addr.ip(), addr.port());

    server.serve(app.into_make_service()).await.unwrap();
    // Code references for database interactions with Diesel, delete later.

    // use api::schema::products::dsl::*;
    //
    // let conn = &mut establish_connection();
    //
    // // let new_products: Vec<NewProduct> = vec![
    // //     NewProduct {
    // //         name: "Brocoli",
    // //         expiration_months: Some(16)
    // //     },
    // //     NewProduct {
    // //         name: "Asperges",
    // //         expiration_months: Some(16)
    // //     }
    // // ];
    // // diesel::insert_into(products)
    // //     .values(new_products)
    // //     .returning(Product::as_returning())
    // //     .get_result(conn)
    // //     .expect("Error saving new product");
    // fn get_product_id(conn: &mut PgConnection, query_name: &str) -> Result<Option<i32>, Error> {
    //     products
    //         .select(product_id)
    //         .filter(name.eq(query_name))
    //         .get_result(conn)
    //         .optional()
    // }
    //
    // let results = products
    //     .select(Product::as_select())
    //     .load(conn)
    //     .expect("Error loading products");
    // for product in results {
    //     println!("{} expires after {} months", product.name, product.expiration_months);
    // }
    //
    // if let Ok(Some(test_name)) = get_product_id(conn, "Brocoli") {
    //     println!("{}", test_name);
    // } else {
    //     println!("Could not find Brocoli");
    // }
    // if let Ok(Some(false_name)) = get_product_id(conn, "Non existent") {
    //     println!("{}", false_name);
    // } else {
    //     println!("Could not find 'Non existent'");
    // }
    //
    // if let Ok(Some(new_product_id)) = get_product_id(conn, "Brocoli") {
    //     let new_storage_item = NewStorageItem {
    //         product_id: new_product_id,
    //         weight_grams: 525.2,
    //         date_in: Local::now().date_naive(),
    //         available: true,
    //     };
    //     diesel::insert_into(storage)
    //         .values(new_storage_item)
    //         .returning(Storage::as_returning())
    //         .get_result(conn)
    //         .expect("Could not add new storage item!");
    // } else {
    //     println!("Could not find the product in the database.");
    // }
}
