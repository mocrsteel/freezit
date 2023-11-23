use crate::common::db_data::PRODUCTS;
use crate::common::db::Context;
use api::app;

use axum::{
    body::Body,
    http::Request,
};
use hyper::StatusCode;
use serde::__private::de::TagOrContentField::Content;
use serde_json::{json, Value};
use tower::util::ServiceExt;
use api::models::{Product, ProductTuple};

static MOD: &str = "router_products";

#[tokio::test]
async fn get_product_by_id() {
    let ctx = Context::new(MOD);
    let app = app(Some(ctx.database_url())).await;
    let query_id = 1;

    let response = app
        .oneshot(Request::builder()
            .uri(format!("/api/products/id={}", query_id))
            .body(Body::empty()).unwrap()
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let product_vec: Vec<ProductTuple> = PRODUCTS
        .into_iter()
        .filter(|(id, name, exp)| {
            id.eq(&query_id)
        })
        .collect();

    let product = Product::from_tuple(product_vec[0]);

    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let response_product: Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(response_product, json!(product));
}

#[tokio::test]
async fn get_product_by_name() {
    let ctx = Context::new(MOD);
    let app = app(Some(ctx.database_url())).await;
    let query_name = "Brocoli";
    let response = app
        .oneshot(Request::builder()
            .uri(format!("/api/products/name={}", query_name))
            .body(Body::empty()).unwrap()
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let products_vec: Vec<ProductTuple> = PRODUCTS
        .into_iter()
        .filter(|(_, name, _)| {
            name.eq(&query_name)
        }).collect();
    let product = Product::from_tuple(products_vec[0]);

    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let response_product: Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(response_product, json!(product));
}

#[tokio::test]
async fn get_products_by_expiration() {
    let ctx = Context::new(MOD);
    let app = app(Some(ctx.database_url())).await;
    let query_expiration = 12;

    let response = app
        .oneshot(Request::builder()
            .uri(format!("/api/products/expiration={}", query_expiration))
            .body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let products_vec: Vec<ProductTuple> = PRODUCTS
        .into_iter()
        .filter(|(_, _, expiration)| {
            expiration.eq(&query_expiration)
        }).collect();
    let products_vec_check = Product::from_vec(products_vec);

    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let response_products: Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(response_products, json!(products_vec_check))
}

#[tokio::test]
async fn update_product() {
    let ctx = Context::new(MOD);
    let app = app(Some(ctx.database_url())).await;

    todo!()

}

#[tokio::test]
async fn delete_product() {
    let ctx = Context::new(MOD);
    let app = app(Some(ctx.database_url())).await;

    todo!()
}