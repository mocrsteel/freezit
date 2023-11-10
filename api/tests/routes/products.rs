use api::app;

use axum::{
    body::Body,
    http::Request,
};
use hyper::StatusCode;
use tower::util::ServiceExt;


#[tokio::test]
async fn get_product_by_id() {
    let app = app().await;

    let response = app
        .oneshot(Request::builder()
            .uri("/api/products/id=1")
            .body(Body::empty()).unwrap()
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn get_product_by_name() {
    let app = app().await;

    let response = app
        .oneshot(Request::builder()
            .uri("/api/products/name=Brocoli")
            .body(Body::empty()).unwrap()
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

}