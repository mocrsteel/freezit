// The code in this test does not work yet. For some reason the Client responds with an OS error 61.

// use std::net::SocketAddr;
//
// use axum::http::StatusCode;
// use hyper::{Body, Client, Request};
//
// use api::app;

// #[tokio::test]
// async fn server_root_endpoint() {
//     // let listener = TcpListener::bind("127.0.0.1").await.unwrap();
//     let addr = SocketAddr::from(([127,0,0,1], 3010));
//
//     tokio::spawn(async move {
//         hyper::Server::bind(&addr)
//             .serve( app().await.into_make_service())
//             .await
//             .unwrap_or_else(|err| {
//                 println!("Error binding to address: {}", err);
//             });
//     });
//
//     let client = Client::new();
//
//     let res = client
//         .request(Request::builder()
//             .uri(format!("http://{addr}/api"))
//             .body(Body::empty())
//             .unwrap()
//         )
//         .await
//         .map_err(|err| {
//             println!("Error while running client request: {}", err);
//         })
//         .unwrap();
//
//     let body = hyper::body::to_bytes(res.into_body()).await.unwrap();
//     assert_eq!(res.status(), StatusCode::OK);
//     assert_eq!(&body[..], b"")
// }

use api::app;

use axum::{
    body::Body,
    http::Request,
};
use hyper::StatusCode;
use tower::util::ServiceExt;

#[tokio::test]
async fn api_root_response() {
    let app = app(None).await;
    let expected_body = b"API active";

    let response = app
        .oneshot(Request::builder()
            .uri("/api")
            .body(Body::empty()).unwrap()
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();

    assert_eq!(&body[..], expected_body);
}