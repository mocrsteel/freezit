use axum::{body::Body, http::{Request, StatusCode}};
use serde_json::ser;
use tower::{Service, ServiceExt};


use api::{app,models::{Freezer, NewFreezer}};
use crate::common::{db::Context, db_data::FREEZERS};


static MOD: &str = "router_freezers";

#[tokio::test]
async fn creates_product_correctly() {
    let ctx = Context::new(MOD);
    let mut app = app(Some(ctx.database_url())).await;

    let new_freezer = NewFreezer {
        name: String::from("Bureau")
    };

    let request = Request::builder()
        .uri("/api/freezers/create")
        .method("POST")
        .header("Content-Type", "application/json")
        .body(Body::from(ser::to_string(&new_freezer).unwrap()))
        .unwrap();
    let create_response = ServiceExt::ready(&mut app)
        .await
        .unwrap()
        .call(request)
        .await
        .unwrap();

    assert_eq!(create_response.status(), StatusCode::OK);

    let get_response = app.oneshot(
        Request::builder()
            .uri(format!("/api/freezers/name={}", new_freezer.name))
            .body(Body::empty())
            .unwrap()
    ).await.unwrap();

    assert_eq!(get_response.status(), StatusCode::OK);

    let bytes = hyper::body::to_bytes(get_response.into_body()).await.unwrap();
    let response_freezer: Freezer = serde_json::from_slice(&bytes).unwrap();

    assert_eq!(response_freezer.name, new_freezer.name);
}

#[tokio::test]
async fn create_returns_error_on_non_unique_name() {
    let ctx = Context::new(MOD);
    let app = app(Some(ctx.database_url())).await;

    let existing_freezer = NewFreezer {
        name: String::from(FREEZERS[1].1)
    };

    let create_response = app.oneshot(
        Request::builder()
            .uri("/api/freezers/create")
            .method("POST")
            .header("Content-Type", "application/json")
            .body(Body::from(ser::to_string(&existing_freezer).unwrap()))
            .unwrap()
    ).await.unwrap();

    assert_eq!(create_response.status(), StatusCode::INTERNAL_SERVER_ERROR);

    let body = hyper::body::to_bytes(create_response.into_body()).await.unwrap();

    // Probably needs fine-tuning in terms of response message.
    assert_eq!(&body[..], b"Unique constraint violation");
}

#[tokio::test]
async fn gets_correct_freezer_by_id() {
    let ctx = Context::new(MOD);
    let app = app(Some(ctx.database_url())).await;

    let expected_freezer = Freezer::from_tuple(FREEZERS[3]);

    let get_response = app.oneshot(
        Request::builder()
            .uri(format!("/api/freezers/id={}", expected_freezer.freezer_id))
            .body(Body::empty())
            .unwrap()
    ).await.unwrap();

    assert_eq!(get_response.status(), StatusCode::OK);

    let bytes = hyper::body::to_bytes(get_response.into_body()).await.unwrap();
    let response_freezer: Freezer = serde_json::from_slice(&bytes).unwrap();

    assert_eq!(response_freezer, expected_freezer);
}

#[tokio::test]
async fn gets_correct_freezer_by_name() {
    let ctx = Context::new(MOD);
    let app = app(Some(ctx.database_url())).await;

    let expected_freezer = Freezer::from_tuple(FREEZERS[3]);

    let get_response = app.oneshot(
        Request::builder()
            .uri(format!("/api/freezers/name={}", expected_freezer.name.replace(" ", "%20")))
            .body(Body::empty())
            .unwrap()
    ).await.unwrap();

    assert_eq!(get_response.status(), StatusCode::OK);

    let bytes = hyper::body::to_bytes(get_response.into_body()).await.unwrap();
    let response_freezer: Freezer = serde_json::from_slice(&bytes).unwrap();

    assert_eq!(response_freezer, expected_freezer);
}

#[tokio::test]
async fn root_gets_all_freezers() {
    let ctx = Context::new(MOD);
    let app = app(Some(ctx.database_url())).await;

    let expected_freezer_vec = Freezer::from_vec(FREEZERS.to_vec());

    let root_response = app.oneshot(
        Request::builder()
            .uri("/api/freezers")
            .body(Body::empty())
            .unwrap()
    ).await.unwrap();

    assert_eq!(root_response.status(), StatusCode::OK);

    let bytes = hyper::body::to_bytes(root_response.into_body()).await.unwrap();
    let response_freezer_vec: Vec<Freezer> = serde_json::from_slice(&bytes).unwrap();

    assert_eq!(response_freezer_vec, expected_freezer_vec);
}

#[tokio::test]
async fn updates_product_correctly() {
    let ctx = Context::new(MOD);
    let mut app = app(Some(ctx.database_url())).await;

    let nonexistent_freezer_name = "Tuinhuis";

    let request = Request::builder()
        .uri("/api/freezers/id=1")
        .body(Body::empty())
        .unwrap();
    let get_response = ServiceExt::ready(&mut app)
        .await
        .unwrap()
        .call(request)
        .await
        .unwrap();

    let bytes = hyper::body::to_bytes(get_response.into_body()).await.unwrap();
    let mut freezer: Freezer = serde_json::from_slice(&bytes).unwrap();
    freezer.name = String::from(nonexistent_freezer_name);

    let request = Request::builder()
        .uri("api/freezers")
        .method("PATCH")
        .header("Content-Type", "application/json")
        .body(Body::from(ser::to_string(&freezer).unwrap()))
        .unwrap();
    let update_response = ServiceExt::ready(&mut app)
        .await
        .unwrap()
        .call(request)
        .await
        .unwrap();

    assert_eq!(update_response.status(), StatusCode::OK);

    let check_update_response = app.oneshot(
        Request::builder()
            .uri(format!("/api/freezers/id={}", freezer.freezer_id))
            .body(Body::empty())
            .unwrap()
    ).await.unwrap();

    let bytes = hyper::body::to_bytes(check_update_response.into_body()).await.unwrap();
    let updated_freezer: Freezer = serde_json::from_slice(&bytes).unwrap();

    assert_eq!(updated_freezer.name, freezer.name);
}


#[tokio::test]
async fn update_returns_error_on_non_unique_name() {
    let ctx = Context::new(MOD);
    let mut app = app(Some(ctx.database_url())).await;

    let existent_freezer_name = FREEZERS[2].1;

    let request = Request::builder()
        .uri("/api/freezers/id=1")
        .body(Body::empty())
        .unwrap();
    let get_response = ServiceExt::ready(&mut app)
        .await
        .unwrap()
        .call(request)
        .await
        .unwrap();

    let bytes = hyper::body::to_bytes(get_response.into_body()).await.unwrap();
    let mut freezer: Freezer = serde_json::from_slice(&bytes).unwrap();
    freezer.name = String::from(existent_freezer_name);

    let request = Request::builder()
        .uri("api/freezers")
        .method("PATCH")
        .header("Content-Type", "application/json")
        .body(Body::from(ser::to_string(&freezer).unwrap()))
        .unwrap();
    let update_response = ServiceExt::ready(&mut app)
        .await
        .unwrap()
        .call(request)
        .await
        .unwrap();

    assert_eq!(update_response.status(), StatusCode::INTERNAL_SERVER_ERROR);

    let body = hyper::body::to_bytes(update_response.into_body()).await.unwrap();

    assert_eq!(&body[..], b"duplicate key value violates unique constraint \"freezers_name_key\"")
}

#[tokio::test]
async fn deletes_product_correctly() {
    let ctx = Context::new(MOD);
    let mut app = app(Some(ctx.database_url())).await;

    let request = Request::builder()
        .uri("/api/freezers/id=1")
        .method("DELETE")
        .body(Body::empty())
        .unwrap();
    let delete_response = ServiceExt::ready(&mut app)
        .await
        .unwrap()
        .call(request)
        .await
        .unwrap();

    assert_eq!(delete_response.status(), StatusCode::OK);

    let check_response = app.oneshot(
        Request::builder()
            .uri("api/freezers/id=1")
            .body(Body::empty())
            .unwrap()
    ).await.unwrap();

    assert_eq!(check_response.status(), StatusCode::INTERNAL_SERVER_ERROR);

    let body = hyper::body::to_bytes(check_response.into_body()).await.unwrap();

    assert_eq!(&body[..], b"Record not found")
}

#[tokio::test]
async fn delete_returns_error_on_nonexistent_id() {
    let ctx = Context::new(MOD);
    let app = app(Some(ctx.database_url())).await;

    let response = app.oneshot(
        Request::builder()
            .uri("/api/freezers/id=10")
            .method("DELETE")
            .body(Body::empty())
            .unwrap()
    ).await.unwrap();

    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);

    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();

    assert_eq!(&body[..], b"Record not found");
}


