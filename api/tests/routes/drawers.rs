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

    let Drawer {drawer_id: _, name, freezer_id } = Drawer::from_tuple(DRAWERS[10]);
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
async fn creates_drawer_correctly_on_existing_name() {
    let ctx = Context::new(MOD);
    let app = app(Some(ctx.database_url())).await;

    let Drawer { drawer_id: _, name, freezer_id} = Drawer::from_tuple(DRAWERS[4]);
    let new_drawer = NewDrawer {
        name,
        freezer_id: 3
    };

    // Check to make sure we set up the test correctly!
    let drawers = Drawer::from_vec(DRAWERS.to_vec());
    let check_drawers = drawers.iter().filter(|Drawer {drawer_id: _, name, freezer_id}| {
        name.eq(&new_drawer.name) && freezer_id.eq(&new_drawer.freezer_id)
    }).collect::<Vec<&Drawer>>();
    assert!(check_drawers.is_empty(), "This freezer_id<->name combination is not unique!");
    assert_ne!(new_drawer.freezer_id, freezer_id, "Error in test setup. freezer_id should be different!");

    // The actual test.
    let create_response = app.oneshot(
        Request::builder()
            .uri("/api/drawers")
            .method("POST")
            .header("Content-Type", "application/json")
            .body(Body::from(serde_json::to_string(&new_drawer).unwrap()))
            .unwrap()
    ).await.unwrap();

    assert_eq!(create_response.status(), StatusCode::OK);

    let body = hyper::body::to_bytes(create_response.into_body()).await.unwrap();
    let response_drawer: Drawer = serde_json::from_slice(&body).unwrap();

    assert_eq!(response_drawer.name, new_drawer.name);
    assert_eq!(response_drawer.freezer_id, new_drawer.freezer_id);
}

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
async fn gets_correct_drawer_by_id() {
    let ctx = Context::new(MOD);
    let app = app(Some(ctx.database_url())).await;

    let expected_drawer = Drawer::from_tuple(DRAWERS[8]);

    let get_response = app.oneshot(
        Request::builder()
            .uri(format!("/api/drawers?drawerId={}", expected_drawer.drawer_id))
            .body(Body::empty())
            .unwrap()
    ).await.unwrap() ;

    assert_eq!(get_response.status(), StatusCode::OK);

    let body = hyper::body::to_bytes(get_response.into_body()).await.unwrap();
    let response_drawers: Vec<Drawer> = serde_json::from_slice(&body).unwrap();

    assert_eq!(response_drawers.len(), 1);
    assert_eq!(response_drawers[0], expected_drawer);
}

#[tokio::test]
async fn gets_correct_drawer_vec_by_name() {
    let ctx = Context::new(MOD);
    let app = app(Some(ctx.database_url())).await;

    let expected_drawers = Drawer::from_vec(DRAWERS.to_vec())
        .into_iter()
        .filter(|Drawer {drawer_id: _, name, freezer_id: _}| {
            name.eq(DRAWERS[10].1)
        })
        .collect::<Vec<Drawer>>();

    let get_response = app.oneshot(
        Request::builder()
            .uri(format!("/api/drawers?drawerName={}", &expected_drawers[0].name).replace(' ', "%20"))
            .body(Body::empty())
            .unwrap()
    ).await.unwrap();

    assert_eq!(get_response.status(), StatusCode::OK);

    let body = hyper::body::to_bytes(get_response.into_body()).await.unwrap();
    let drawers: Vec<Drawer> = serde_json::from_slice(&body).unwrap();

    assert_eq!(drawers, expected_drawers);
}

#[tokio::test]
async fn gets_correct_drawers_vec_by_freezer_id() {
    let ctx = Context::new(MOD);
    let app = app(Some(ctx.database_url())).await;

    let expected_drawers = Drawer::from_vec(DRAWERS.to_vec())
        .into_iter()
        .filter(|Drawer {drawer_id: _, name: _, freezer_id}| {
            freezer_id.eq(&DRAWERS[3].2)
        })
        .collect::<Vec<Drawer>>();

    let get_response = app.oneshot(
        Request::builder()
            .uri(format!("/api/drawers?freezerId={}", &expected_drawers[0].freezer_id))
            .body(Body::empty())
            .unwrap()
    ).await.unwrap();

    assert_eq!(get_response.status(), StatusCode::OK);

    let body = hyper::body::to_bytes(get_response.into_body()).await.unwrap();
    let response_drawers: Vec<Drawer> = serde_json::from_slice(&body).unwrap();

    assert_eq!(response_drawers, expected_drawers);
}

#[tokio::test]
async fn gets_correct_drawer_by_name_freezer_id_combination() {
    let ctx = Context::new(MOD);
    let app = app(Some(ctx.database_url())).await;

    let expected_drawers = vec![Drawer::from_tuple(DRAWERS[7])];

    let get_response = app.oneshot(
        Request::builder()
            .uri(format!("/api/drawers?drawerName={}&freezerId={}", &expected_drawers[0].name, &expected_drawers[0].freezer_id).replace(' ', "%20"))
            .body(Body::empty())
            .unwrap()
    ).await.unwrap();

    assert_eq!(get_response.status(), StatusCode::OK);

    let body = hyper::body::to_bytes(get_response.into_body()).await.unwrap();
    let result_drawers: Vec<Drawer> = serde_json::from_slice(&body).unwrap();

    assert_eq!(result_drawers.len(), 1);
    assert_eq!(result_drawers, expected_drawers);
}

#[tokio::test]
async fn get_returns_error_on_invalid_query_parameter_value_type() {
    let ctx = Context::new(MOD);
    let app = app(Some(ctx.database_url())).await;

    let get_response = app.oneshot(
        Request::builder()
            .uri("/api/drawers?drawerId=InvalidValueType")
            .body(Body::empty())
            .unwrap()
    ).await.unwrap();

    assert_eq!(get_response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn updates_drawer_correctly() {
    let ctx = Context::new(MOD);
    let mut app = app(Some(ctx.database_url())).await;

    let get_response = ServiceExt::ready(&mut app)
        .await
        .unwrap()
        .call(
            Request::builder()
                .uri("/api/drawers?drawerId=1")
                .body(Body::empty())
                .unwrap()
        ).await.unwrap();
    let body = hyper::body::to_bytes(get_response.into_body()).await.unwrap();
    let mut update_drawers: Vec<Drawer> = serde_json::from_slice(&body).unwrap();
    update_drawers[0].name = String::from("test update");

    let update_request = Request::builder()
        .uri("/api/drawers")
        .method("PATCH")
        .header("Content-Type", "application/json")
        .body(Body::from(serde_json::to_string(&update_drawers[0]).unwrap()))
        .unwrap();
    let update_response = ServiceExt::ready(&mut app)
        .await
        .unwrap()
        .call(update_request)
        .await
        .unwrap();

    assert_eq!(update_response.status(), StatusCode::OK);

    let check_response = ServiceExt::ready(&mut app)
        .await
        .unwrap()
        .call(
            Request::builder()
                .uri("/api/drawers?drawerName=test%20update")
                .body(Body::empty())
                .unwrap()
        )
        .await
        .unwrap();

    assert_eq!(check_response.status(), StatusCode::OK);

    let body = hyper::body::to_bytes(check_response.into_body()).await.unwrap();
    let check_drawer: Vec<Drawer> = serde_json::from_slice(&body).unwrap();

    assert_eq!(check_drawer, update_drawers);
}

#[tokio::test]
async fn update_returns_error_on_existing_name_freezer_id_combination() {
    let ctx = Context::new(MOD);
    let mut app = app(Some(ctx.database_url())).await;

    let get_response = ServiceExt::ready(&mut app)
        .await
        .unwrap()
        .call(
            Request::builder()
                .uri("/api/drawers?drawerId=1")
                .body(Body::empty())
                .unwrap()
        )
        .await
        .unwrap();
    let body = hyper::body::to_bytes(get_response.into_body()).await.unwrap();
    let mut drawers: Vec<Drawer> = serde_json::from_slice(&body).unwrap();
    drawers[0].name = String::from("Schuif 2");

    let request = Request::builder()
        .uri("/api/drawers")
        .method("PATCH")
        .header("Content-Type", "application/json")
        .body(Body::from(serde_json::to_string(&drawers[0]).unwrap()))
        .unwrap();
    let update_response = ServiceExt::ready(&mut app)
        .await
        .unwrap()
        .call(request)
        .await
        .unwrap();

    assert_eq!(update_response.status(), StatusCode::INTERNAL_SERVER_ERROR);

    let body = hyper::body::to_bytes(update_response.into_body()).await.unwrap();
    let error_text = std::str::from_utf8(&body).unwrap();

    assert_eq!(error_text, "This drawer name already exists within this freezer")
}

#[tokio::test]
async fn updates_drawer_name_correctly_on_existing_name_in_other_freezer() {
    let ctx = Context::new(MOD);
    let mut app = app(Some(ctx.database_url())).await;

    let get_response = ServiceExt::ready(&mut app)
        .await
        .unwrap()
        .call(
            Request::builder()
                .uri("/api/drawers?drawerName=Schuif%201&freezerId=3")
                .body(Body::empty())
                .unwrap()
        )
        .await
        .unwrap();
    let body = hyper::body::to_bytes(get_response.into_body()).await.unwrap();
    let mut drawers: Vec<Drawer> = serde_json::from_slice(&body).unwrap();
    drawers[0].name = String::from("Schuif 5");

    let request = Request::builder()
        .uri("/api/drawers")
        .method("PATCH")
        .header("Content-Type", "application/json")
        .body(Body::from(serde_json::to_string(&drawers[0]).unwrap()))
        .unwrap();
    let update_response = ServiceExt::ready(&mut app)
        .await
        .unwrap()
        .call(request)
        .await
        .unwrap();

    assert_eq!(update_response.status(), StatusCode::OK);
}

#[tokio::test]
async fn deletes_drawer_correctly() {
    let ctx = Context::new(MOD);
    let mut app = app(Some(ctx.database_url())).await;

    let delete_response = ServiceExt::ready(&mut app)
        .await
        .unwrap()
        .call(
            Request::builder()
                .uri("/api/drawers/1")
                .method("DELETE")
                .body(Body::empty())
                .unwrap()
        )
        .await
        .unwrap();

    assert_eq!(delete_response.status(), StatusCode::OK);

    let check_get_response = ServiceExt::ready(&mut app)
        .await
        .unwrap()
        .call(
            Request::builder()
                .uri("/api/drawers?drawerId=1")
                .body(Body::empty())
                .unwrap()
        )
        .await
        .unwrap();

    assert_eq!(check_get_response.status(), StatusCode::OK);

    let body = hyper::body::to_bytes(check_get_response.into_body()).await.unwrap();
    let get_drawers: Vec<Drawer> = serde_json::from_slice(&body).unwrap();

    assert_eq!(get_drawers.len(), 0);
}

#[tokio::test]
async fn delete_returns_error_on_nonexistent_drawer_id() {
    let ctx = Context::new(MOD);
    let app = app(Some(ctx.database_url())).await;

    let delete_response = app.oneshot(
        Request::builder()
            .uri("/api/drawers/100")
            .method("DELETE")
            .body(Body::empty())
            .unwrap()
    ).await.unwrap();

    assert_eq!(delete_response.status(), StatusCode::INTERNAL_SERVER_ERROR);

    let body = hyper::body::to_bytes(delete_response.into_body()).await.unwrap();
    let error_text = std::str::from_utf8(&body).unwrap();

    assert_eq!(error_text, "Drawer not found");
}
