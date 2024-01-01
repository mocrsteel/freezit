use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use tower::{Service, ServiceExt};

use api::{
    app,
    models::{Drawer, NewDrawer},
};
use crate::common::db::Context;
use crate::common::db_data::{FREEZERS, DRAWERS};

static MOD: &str = "router_drawers";

#[tokio::test]
async fn creates_drawer_correctly() {
    let ctx = Context::new(MOD);
    let mut app = app(Some(ctx.database_url())).await;

    let new_drawer = NewDrawer {
        name: String::from("New Drawer"),
        freezer_id: FREEZERS[0].0
    };
    let post_request = Request::builder()
        .uri("/api/drawers")
        .method("POST")
        .header("Content-Type", "application/json")
        .body(Body::from(serde_json::to_string(&new_drawer).unwrap()))
        .unwrap();

    let create_response = ServiceExt::ready(&mut app)
        .await
        .unwrap()
        .call(post_request)
        .await
        .unwrap();

    assert_eq!(create_response.status(), StatusCode::OK);

    let get_request = Request::builder()
        .uri(format!("/api/drawers?drawerName={}&freezerId={}", new_drawer.name, new_drawer.freezer_id).replace(' ', "%20"))
        .method("GET")
        .body(Body::empty())
        .unwrap();
    let get_response = ServiceExt::ready(&mut app)
        .await
        .unwrap()
        .call(get_request)
        .await
        .unwrap();

    assert_eq!(get_response.status(), StatusCode::OK);

    let body = hyper::body::to_bytes(get_response.into_body()).await.unwrap();
    let response_drawers: Vec<Drawer> = serde_json::from_slice(&body).unwrap();

    assert_eq!(response_drawers.len(), 1);
    assert_eq!(response_drawers[0].name, new_drawer.name);
    assert_eq!(response_drawers[0].freezer_id, new_drawer.freezer_id);
}

#[tokio::test]
async fn returns_error_on_create_existing_name_freezer_id_combination() {
    let ctx = Context::new(MOD);
    let app = app(Some(ctx.database_url())).await;

    let Drawer {drawer_id, name, freezer_id } = Drawer::from_tuple(DRAWERS[10]);
    let error_drawer = NewDrawer {
        name,
        freezer_id
    };

    let post_response = app.oneshot(
        Request::builder()
            .uri("/api/drawers")
            .method("POST")
            .header("Content-Type", "application/json")
            .body(Body::from(serde_json::to_string(&error_drawer).unwrap()))
            .unwrap()
    ).await.unwrap();

    assert_eq!(post_response.status(), StatusCode::INTERNAL_SERVER_ERROR);

    let response_body = hyper::body::to_bytes(post_response.into_body()).await.unwrap();
    let response_text = std::str::from_utf8(&response_body[..]).unwrap();

    assert_eq!(response_text, "This drawer name already exists within this freezer")
}

#[tokio::test]
async fn creates_drawer_correctly_on_existing_name() { todo!() }

#[tokio::test]
async fn gets_all_drawers_without_query_params() {
    let ctx = Context::new(MOD);
    let app = app(Some(ctx.database_url())).await;

    let expected_drawer_vec = Drawer::from_vec(DRAWERS.to_vec());

    let get_response = app.oneshot(
        Request::builder()
            .uri("/api/drawers")
            .body(Body::empty())
            .unwrap()
    ).await.unwrap();

    assert_eq!(get_response.status(), StatusCode::OK);

    let body = hyper::body::to_bytes(get_response.into_body()).await.unwrap();
    let response_vec: Vec<Drawer> = serde_json::from_slice(&body).unwrap();

    assert_eq!(response_vec, expected_drawer_vec);
}

#[tokio::test]
async fn gets_all_drawers_on_invalid_params() {
    let ctx = Context::new(MOD);
    let app = app(Some(ctx.database_url())).await;

    let get_response = app.oneshot(
        Request::builder()
            .uri("/api/drawers?invalidQueryParameter=2")
            .body(Body::empty())
            .unwrap()
    ).await.unwrap();

    assert_eq!(get_response.status(), StatusCode::OK);

    let body = hyper::body::to_bytes(get_response.into_body()).await.unwrap();
    let result_vec: Vec<Drawer> = serde_json::from_slice(&body).unwrap();

    assert_eq!(result_vec.len(), DRAWERS.len());
}

#[tokio::test]
async fn gets_correct_drawer_by_id() { todo!() }

#[tokio::test]
async fn gets_correct_drawer_vec_by_name() { todo!() }

#[tokio::test]
async fn gets_correct_drawers_vec_by_freezer_id() { todo!() }

#[tokio::test]
async fn gets_correct_drawer_by_name_freezer_id_combination() { todo!() }

#[tokio::test]
async fn get_returns_error_on_invalid_query_parameter_value_type() { todo!() }

#[tokio::test]
async fn updates_drawer_correctly() { todo!() }

#[tokio::test]
async fn update_returns_error_on_existing_name_freezer_id_combination() { todo!() }

#[tokio::test]
async fn updates_drawer_correctly_on_existing_name() { todo!() }

#[tokio::test]
async fn deletes_drawer_correctly() { todo!() }

#[tokio::test]
async fn delete_returns_error_on_nonexistent_drawer_id() { todo!() }
