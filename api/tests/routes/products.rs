use crate::common::db_data::PRODUCTS;
use crate::common::db::Context;
use api::app;

use axum::{
    body::Body,
    http::Request,
};
use hyper::StatusCode;
use serde_json::{json, Value};
use tower::util::ServiceExt;
use api::models::{NewProduct, Product, ProductTuple};

static MOD: &str = "router_products";

#[tokio::test]
async fn get_product_by_id() {
    let ctx = Context::new(MOD);
    let app = app(Some(ctx.database_url())).await;
    let query_id = 1;

    let response = app
        .oneshot(Request::builder()
            .uri(format!("/api/products?id={}", query_id))
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
            .uri(format!("/api/products?name={}", query_name))
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
            .uri(format!("/api/products?expiration={}", query_expiration))
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
async fn create_product() {
    let ctx = Context::new(MOD);
    let app = app(Some(ctx.database_url())).await;
    let new_product = NewProduct {
        name: "New Produce",
        expiration_months: Some(24),
    };
    let result = app
        .oneshot(Request::builder()
            .uri("/api/products/create")
            .method("POST")
            .body(Body::from(json!(new_product)))
            .unwrap()
        )
        .await
        .unwrap();

    assert!(result.status(), StatusCode::OK);

    let get_product = app
        .oneshot(Request::builder()
            .uri(format!("/api/products?name={}", new_product.name))
            .body(Body::empty())
            .unwrap()
        )
        .await
        .unwrap();

    let body = hyper::body::to_bytes(get_product.into_body()).await.unwrap();
    let response_product: Product = serde_json::from_slice(&body).unwrap();

    assert_eq!(response_product.name, new_product.name);
}

#[tokio::test]
async fn cannot_create_existing_product() {
    let ctx = Context::new(MOD);
    let app = app(Some(ctx.database_url())).await;
    let new_product = NewProduct {
        name: "Brocoli",
        expiration_months: Some(24),
    };

    let response = app
        .oneshot(Request::builder()
            .uri("/api/products/create")
            .method("POST")
            .body(Body::from(json!(new_product)))
            .unwrap()
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    assert_eq!(response.status().as_str(), "This product already exists.");
}

#[tokio::test]
async fn update_product() {
    let ctx = Context::new(MOD);
    let app = app(Some(ctx.database_url())).await;
    let product_name = "Brocoli";

    let get_response = app
        .oneshot(Request::builder()
            .uri(format!("/api/products?name={}", product_name))
            .body(Body::empty())
            .unwrap()
        )
        .await
        .unwrap();

    let body = hyper::body::to_bytes(get_response.into_body()).await.unwrap();
    let mut product: Product = serde_json::from_slice(&body).unwrap();
    product.name = String::from("Geen Brocoli");

    let update_response = app
        .oneshot(Request::builder()
            .uri("/api/products")
            .method("UPDATE")
            .body(Body::from(json!(product)))
            .unwrap()
        )
        .await
        .unwrap();

    assert_eq!(update_response.status(), StatusCode::OK);

    let check_get_response = app
        .onshot(Request::builder()
            .uri(format!("/api/products?name={}", product.name))
            .body(Body::empty())
            .unwrap()
        )
        .await
        .unwrap();

    assert_eq!(check_get_response.status(), StatusCode::OK);
}

#[tokio::test]
async fn cannot_change_product_name_to_existing() {
    let ctx = Context::new(MOD);
    let app = app(Some(ctx.database_url())).await;
    let product_name = "Groentensoep";

    let mut product: Product = serde_json::from_slice(
        hyper::body::to_bytes(
            app.oneshot(Request::builder()
                .uri(format!("/api/products?name={}", product_name))
                .body(Body::empty())
                .unwrap()
            )
                .await
                .unwrap()
                .into_body()
        ).unwrap()
    ).await.unwrap();

    // just a check
    assert_eq!(product.name, String::from(product_name));

    product.name = String::from("Brocoli");

    let response = app.oneshot(
        Request::builder()
            .uri("/api/products")
            .methode("UPDATE")
            .body(Body::from(json!(product)))
            .unwrap()
    )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    assert_eq!(response.status().as_str(), "This project already exists.")
}

#[tokio::test]
async fn delete_product() {
    let ctx = Context::new(MOD);
    let app = app(Some(ctx.database_url())).await;
    let id = 1;

    let response = app.oneshot(
        Request::builder()
            .uri(format!("/api/products?id={}", id))
            .method("DELETE")
            .body(Body::empty())
            .unwrap()
    )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let result_query = app.oneshot(
        Request::builer()
            .uri(format!("/api/products?id={}", id))
            .body(Body::empty())
            .unwrap()
    )
        .await
        .unwrap();

    assert_eq!(result_query.status(), StatusCode::INTERNAL_SERVER_ERROR);
    assert_eq!(result_query.status().as_str(), "Could not find the requested product.");
}