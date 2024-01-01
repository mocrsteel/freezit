use crate::common::db_data::PRODUCTS;
use crate::common::db::Context;
use api::app;

use log::{error, info};
use axum::{
    body::Body,
    http::Request,
};
use hyper::StatusCode;
use serde_json::{json, Value};
use tower::{Service, ServiceExt};
use api::models::{NewProduct, Product, ProductTuple};

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
        .filter(|(id, _name, _exp)| {
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
async fn get_all_products() {
    let ctx = Context::new(MOD);
    let app = app(Some(ctx.database_url())).await;
    let expected_response = Product::from_vec(PRODUCTS.to_vec());
    let response = app.oneshot(
            Request::builder()
                .uri("/api/products")
                .method("GET")
                .body(Body::empty())
                .unwrap()
    ).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let products: Vec<Product> = serde_json::from_slice(&body).unwrap();

    assert_eq!(products.len(), PRODUCTS.len());
    assert_eq!(products, expected_response);
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
async fn create_product_simple_test() {
    let ctx = Context::new(MOD);
    let app = app(Some(ctx.database_url())).await;
    let new_product = NewProduct {
        name: String::from("New Produce"),
        expiration_months: Some(24),
    };
    info!(target: "create_product", "{:?}", new_product);
    let new_product_json = serde_json::to_string(&new_product).unwrap();
    let request = app.oneshot(Request::builder()
        .uri("/api/products/create")
        .method("POST")
        .header("Content-Type", "application/json")
        .body(Body::from(new_product_json))
        .unwrap()).await.unwrap();

    assert_eq!(request.status(), StatusCode::OK);
}

#[tokio::test]
async fn create_product() {
    let ctx = Context::new(MOD);
    let mut app = app(Some(ctx.database_url())).await;
    let new_product = NewProduct {
        name: String::from("New Produce"),
        expiration_months: Some(24),
    };
    info!(target: "create_product", "{:?}", new_product);
    let request = Request::builder()
        .uri("/api/products/create")
        .method("POST")
        .header("Content-Type", "application/json")
        .body(Body::from(serde_json::to_string(&new_product).unwrap()))
        .unwrap();

    let result = ServiceExt::<Request<Body>>::ready(&mut app)
        .await
        .unwrap()
        .call(request)
        .await
        .unwrap();

    info!(target: "create_product", "{:?}",result.status());

    let get_product = app
        .oneshot(Request::builder()
            .uri(format!("/api/products/name={}", new_product.name.replace(' ', "%20")))
            .body(Body::empty())
            .unwrap()
        )
        .await
        .unwrap();

    let body = hyper::body::to_bytes(get_product.into_body()).await.unwrap();
    let response_product: Product = serde_json::from_slice(&body).unwrap();
    info!(target: "response_product", "{:?}", response_product);

    assert_eq!(response_product.name, new_product.name);
}

#[tokio::test]
async fn cannot_create_existing_product() {
    let ctx = Context::new(MOD);
    let app = app(Some(ctx.database_url())).await;
    let new_product = NewProduct {
        name: String::from("Brocoli"),
        expiration_months: Some(24),
    };

    let response = app
        .oneshot(Request::builder()
            .uri("/api/products/create")
            .method("POST")
            .header("Content-Type", "application/json")
            .body(Body::from(serde_json::to_string(&new_product).unwrap().replace(' ', "%20")))
            .unwrap()
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);

    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let error_text = std::str::from_utf8(&body[..]).unwrap();

    assert_eq!(error_text, "This product name already exists");
}

#[tokio::test]
async fn update_product() {
    let ctx = Context::new(MOD);
    let mut app = app(Some(ctx.database_url())).await;
    let product_name = "Brocoli";

    let request = Request::builder()
        .uri(format!("/api/products/name={}", product_name.replace(' ', "%20")))
        .body(Body::empty())
        .unwrap();

    let get_response = ServiceExt::ready(&mut app)
        .await
        .unwrap()
        .call(request)
        .await
        .unwrap();

    let body = hyper::body::to_bytes(get_response.into_body()).await.unwrap();
    let mut product: Product = serde_json::from_slice(&body).unwrap();
    product.name = String::from("Geen Brocoli");

    let request = Request::builder()
        .uri("/api/products")
        .method("PATCH")
        .header("Content-Type","application/json")
        .body(Body::from(serde_json::to_string(&product).unwrap()))
        .unwrap();

    let update_response = ServiceExt::ready(&mut app)
        .await
        .unwrap()
        .call(request)
        .await
        .unwrap();

    assert_eq!(update_response.status(), StatusCode::OK);

    let check_get_response = app
        .oneshot(Request::builder()
            .uri(format!("/api/products/name={}", product.name.replace(' ', "%20")))
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
    let mut app = app(Some(ctx.database_url())).await;
    let product_name = PRODUCTS[1].1;
    let other_product_name = PRODUCTS[3].1;

    let product_request = Request::builder()
        .uri(format!("/api/products/name={}", product_name))
        .body(Body::empty())
        .unwrap();

    let request_res = ServiceExt::ready(&mut app)
        .await
        .unwrap()
        .call(product_request)
        .await
        .unwrap();

    let body = hyper::body::to_bytes(request_res.into_body()).await.unwrap();
    let mut product: Product = serde_json::from_slice(&body).unwrap();

    // just a check
    assert_eq!(product.name, String::from(product_name));

    product.name = String::from(other_product_name);

    let request = Request::builder()
        .uri("/api/products")
        .method("PATCH")
        .header("Content-Type", "application/json")
        .body(Body::from(serde_json::ser::to_string(&product).unwrap()))
        .unwrap();

    let response = ServiceExt::ready(&mut app)
        .await
        .unwrap()
        .call(request)
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);

    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let error_text = std::str::from_utf8(&body[..]).unwrap();

    assert_eq!(error_text, "This product name already exists");
}

#[tokio::test]
async fn delete_product() {
    let ctx = Context::new(MOD);
    let mut app = app(Some(ctx.database_url())).await;
    let id = 1;

    let delete_request = Request::builder()
        .uri(format!("/api/products/id={}", id))
        .method("DELETE")
        .body(Body::empty())
        .unwrap();

    let response = ServiceExt::ready(&mut app)
        .await
        .unwrap()
        .call(delete_request)
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let get_request = Request::builder()
        .uri(format!("/api/products/id={}", id))
        .body(Body::empty())
        .unwrap();

    let result_query = ServiceExt::ready(&mut app)
        .await
        .unwrap()
        .call(get_request)
        .await
        .unwrap();

    assert_eq!(result_query.status(), StatusCode::INTERNAL_SERVER_ERROR, "{:?}", result_query.status().canonical_reason());
}

#[tokio::test]
async fn delete_nonexistent_product_returns_error() {
    let ctx = Context::new(MOD);
    let app = app(Some(ctx.database_url())).await;

    let res = app
        .oneshot(
            Request::builder()
                .uri("/api/products/id=100")
                .method("DELETE")
                .body(Body::empty())
                .unwrap()
        ).await.unwrap();

    assert_eq!(&res.status(), &StatusCode::INTERNAL_SERVER_ERROR);

    let body = hyper::body::to_bytes(res.into_body()).await.unwrap();
    let error_text = std::str::from_utf8(&body[..]).unwrap();

    assert_eq!(error_text, "This product id does not exist");
}