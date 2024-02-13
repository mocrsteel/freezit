use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use chrono::{Local, Months};
use tower::{Service, ServiceExt};

use api::{
    app, models::{Drawer, Freezer, NewStorageItem, Product, Storage}, routes::storage::StorageResponse,
};

use crate::common::db::Context;
use crate::common::db_data::{DRAWERS, FREEZERS, PRODUCTS, STORAGE};


fn storage_response_from_storage_item(storage: Storage) -> Vec<StorageResponse> {
    let product = &Product::from_vec(PRODUCTS.to_vec())
        .into_iter()
        .filter(|product| product.product_id.eq(&storage.product_id))
        .collect::<Vec<Product>>()[0];
    let drawer = &Drawer::from_vec(DRAWERS.to_vec())
        .into_iter()
        .filter(|drawer| drawer.drawer_id.eq(&storage.drawer_id))
        .collect::<Vec<Drawer>>()[0];
    let freezer = &Freezer::from_vec(FREEZERS.to_vec())
        .into_iter()
        .filter(|freezer| freezer.freezer_id.eq(&drawer.freezer_id))
        .collect::<Vec<Freezer>>()[0];

    StorageResponse::from_query_result(vec![(
        storage,
        product.clone(),
        drawer.clone(),
        freezer.clone(),
    )])
}

fn storage_response_from_storage_vec(storage: Vec<Storage>) -> Vec<StorageResponse> {
    storage
        .iter()
        .map(|storage| storage_response_from_storage_item(storage.clone())[0].clone())
        .collect::<Vec<StorageResponse>>()
}

enum Mod {
    Get,
    Create,
    Update,
    Withdraw,
    Filter,
    Delete,
}

impl Mod {
    pub fn as_str<'a>(&self) -> &'a str {
        match self {
            Self::Get => "storage_get",
            Self::Create => "storage_create",
            Self::Update => "storage_update",
            Self::Withdraw => "storage_withdraw",
            Self::Delete => "storage_delete",
            Self::Filter => "storage_filter",
        }
    }
}

#[tokio::test]
async fn get_storage_by_id_returns_correct_item() {
    let ctx = Context::new(Mod::Get.as_str());
    let app = app(Some(ctx.database_url())).await;

    let storage_item = Storage::from_tuple(STORAGE[20]);
    let expected_response = storage_response_from_storage_item(storage_item.clone());
    let storage_response = app
        .oneshot(
            Request::builder()
                .uri(format!("/api/storage/{}", storage_item.storage_id))
                .body(Body::empty())
                .unwrap(),
        )
        .await.unwrap();

    assert!(storage_response.status().is_success());


    let bytes = hyper::body::to_bytes(storage_response.into_body()).await.unwrap();
    let response_vec = serde_json::from_slice::<Vec<StorageResponse>>(&bytes).unwrap();

    assert_eq!(response_vec.len(), 1);
    assert_eq!(response_vec, expected_response);
}

#[tokio::test]
async fn get_storage_by_id_returns_error_when_not_found() {
    let ctx = Context::new(Mod::Get.as_str());
    let app = app(Some(ctx.database_url())).await;

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/storage/300")
                .body(Body::empty())
                .unwrap(),
        )
        .await.unwrap();

    assert!(
        response.status().is_server_error(),
        "Expected internal server error"
    );

    let bytes = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let error_msg = std::str::from_utf8(&bytes[..]).unwrap();

    assert_eq!(error_msg, "Storage item not found");
}

#[tokio::test]
async fn creates_storage_correctly() {
    let ctx = Context::new(Mod::Create.as_str());
    let mut app = app(Some(ctx.database_url())).await;

    let product = Product::from_tuple(PRODUCTS[4]);
    let drawer = Drawer::from_tuple(DRAWERS[10]);
    let freezer = &Freezer::from_vec(FREEZERS.to_vec())
        .into_iter()
        .filter(|freezer| freezer.freezer_id.eq(&drawer.freezer_id))
        .collect::<Vec<Freezer>>()[0];

    let new_storage =
        NewStorageItem::from(product.product_id, drawer.drawer_id, 325.5, Local::now().date_naive());

    let create_response = ServiceExt::ready(&mut app)
        .await.unwrap()
        .call(
            Request::builder()
                .uri("/api/storage")
                .method("POST")
                .header("Content-Type", "application/json")
                .body(Body::from(serde_json::to_string(&new_storage).unwrap()))
                .unwrap(),
        )
        .await.unwrap();

    assert!(
        create_response.status().is_success(),
        "Expected succesful storage creation"
    );

    let bytes = hyper::body::to_bytes(create_response.into_body())
        .await
        .unwrap();
    let storage_response = serde_json::from_slice::<Vec<StorageResponse>>(&bytes).unwrap();

    assert_eq!(
        storage_response.len(),
        1,
        "Expected the result to only contain a single value"
    );

    let get_response = ServiceExt::ready(&mut app)
        .await.unwrap()
        .call(
            Request::builder()
                .uri(format!("/api/storage/{}", storage_response[0].storage_id))
                .body(Body::empty())
                .unwrap(),
        )
        .await.unwrap();

    assert!(get_response.status().is_success());

    // Add return object check!
    let bytes = hyper::body::to_bytes(get_response.into_body())
        .await
        .unwrap();
    let response_vec = serde_json::from_slice::<Vec<StorageResponse>>(&bytes).unwrap();
    let storage_response = &response_vec[0];

    assert_eq!(storage_response.product_name, product.name);
    assert_eq!(storage_response.freezer_name, freezer.name);
    assert_eq!(storage_response.drawer_name, drawer.name);
    assert_eq!(
        storage_response.in_storage_since,
        Local::now().date_naive()
    );
    assert!(storage_response.weight_grams - 325.5 <= 1e-6);
}

#[tokio::test]
async fn get_storage_root_returns_all_storage() {
    let ctx = Context::new(Mod::Get.as_str());
    let app = app(Some(ctx.database_url())).await;

    let storage_available = Storage::from_vec(STORAGE.to_vec())
        .into_iter()
        .filter(|storage| storage.available)
        .collect::<Vec<Storage>>();
    let storage_unavailable = Storage::from_vec(STORAGE.to_vec())
        .into_iter()
        .filter(|storage| !storage.available)
        .collect::<Vec<Storage>>();
    let expected_vec = storage_response_from_storage_vec(storage_available);
    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/storage")
                .body(Body::empty())
                .unwrap(),
        )
        .await.unwrap();

    assert!(response.status().is_success());

    let bytes = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let result_vec = serde_json::from_slice::<Vec<StorageResponse>>(&bytes).unwrap();

    assert_eq!(result_vec.len(), expected_vec.len());
    // The return should not contain the unavailable storage, but we also want to check we still have enough 
    // and everyting adds up like expected.
    assert_eq!(result_vec.len(), STORAGE.len() - storage_unavailable.len());

    // was required to troubleshoot non-matching vecs. This actually shows that order by is working or not.
    // Left in here as it makes sense to keep it. It's a bit more verbose which allows for better debugging.
    let result = expected_vec.iter().map(|storage| {
        result_vec[storage.storage_id as usize - 1].eq(storage)
    }).collect::<Vec<bool>>();

    let _false_objects = result.iter().enumerate().filter_map(|(i, ok)| {
        if !ok {
            Some((result_vec[i].clone(), expected_vec[i].clone()))
        } else {
            None
        }
    })
        .collect::<Vec<(StorageResponse, StorageResponse)>>();

    assert_eq!(result_vec, expected_vec);
}

#[tokio::test]
async fn updates_storage_correctly() {
    let ctx = Context::new(Mod::Update.as_str());
    let mut app = app(Some(ctx.database_url())).await;

    let query_result = ServiceExt::ready(&mut app)
        .await.unwrap()
        .call(
            Request::builder()
                .uri("/api/storage/25")
                .body(Body::empty())
                .unwrap()
        ).await.unwrap();

    let bytes = hyper::body::to_bytes(query_result.into_body()).await.unwrap();
    let mut storage = serde_json::from_slice::<Vec<StorageResponse>>(&bytes).unwrap()[0].clone();
    let product_name = PRODUCTS[3].1; // Name
    storage.product_name = String::from(product_name);

    let update_response = ServiceExt::ready(&mut app)
        .await.unwrap()
        .call(
            Request::builder()
                .uri("/api/storage")
                .method("PATCH")
                .header("Content-Type", "application/json")
                .body(Body::from(serde_json::to_string(&storage).unwrap()))
                .unwrap()
        )
        .await.unwrap();

    assert!(update_response.status().is_success());

    let bytes = hyper::body::to_bytes(update_response.into_body()).await.unwrap();
    let update_result = &serde_json::from_slice::<Vec<StorageResponse>>(&bytes).unwrap()[0];

    assert_eq!(update_result.storage_id, 25);
    assert_eq!(update_result.product_name, product_name);

    let check_db_response = ServiceExt::ready(&mut app)
        .await.unwrap()
        .call(
            Request::builder()
                .uri("/api/storage/25")
                .body(Body::empty()).unwrap()
        ).await.unwrap();

    let bytes = hyper::body::to_bytes(check_db_response.into_body()).await.unwrap();
    let check_result = &serde_json::from_slice::<Vec<StorageResponse>>(&bytes).unwrap()[0];

    assert_eq!(check_result.storage_id, 25);
    assert_eq!(check_result.product_name, product_name);
}

#[tokio::test]
async fn update_storage_returns_error_when_not_found() {
    let ctx = Context::new(Mod::Update.as_str());
    let app = app(Some(ctx.database_url())).await;

    let storage = Storage::from_tuple(STORAGE[15]);
    let mut storage_response = storage_response_from_storage_item(storage)[0].clone();
    storage_response.storage_id = 300;
    storage_response.product_name = String::from(PRODUCTS[1].1);

    let update_response = app.oneshot(
        Request::builder()
            .uri("/api/storage")
            .method("PATCH")
            .header("Content-Type", "application/json")
            .body(Body::from(serde_json::to_string(&storage_response).unwrap()))
            .unwrap()
    ).await.unwrap();

    assert!(update_response.status().is_server_error());

    let bytes = hyper::body::to_bytes(update_response.into_body()).await.unwrap();
    let error = std::str::from_utf8(&bytes[..]).unwrap();

    assert_eq!(error, "Storage item not found")
}

#[tokio::test]
async fn withdraw_updates_storage_correctly() {
    let ctx = Context::new(Mod::Withdraw.as_str());
    let mut app = app(Some(ctx.database_url())).await;

    let withdraw_response = ServiceExt::ready(&mut app)
        .await.unwrap()
        .call(
            Request::builder()
                .uri("/api/storage/1/withdraw")
                .method("PATCH")
                .body(Body::empty())
                .unwrap()
        ).await.unwrap();

    let (parts, body) = withdraw_response.into_parts();
    let bytes = hyper::body::to_bytes(body).await.unwrap();
    let _response_detail = std::str::from_utf8(&bytes[..]).unwrap();

    assert_eq!(parts.status, StatusCode::OK);

    let check_response = ServiceExt::ready(&mut app)
        .await.unwrap()
        .call(
            Request::builder()
                .uri("/api/storage")
                .body(Body::empty())
                .unwrap()
        ).await.unwrap();

    assert!(check_response.status().is_success());

    let storage_vec = serde_json::from_slice::<Vec<StorageResponse>>(
        &hyper::body::to_bytes(check_response.into_body()).await.unwrap()
    )
        .unwrap()
        .into_iter()
        .filter(|storage| {
            storage.storage_id.eq(&1)
        })
        .collect::<Vec<StorageResponse>>();

    assert!(storage_vec.is_empty(), "Storage item with id 1 was not withdrawn correctly!");
}

#[tokio::test]
async fn withdraw_storage_returns_error_when_not_found() {
    let ctx = Context::new(Mod::Withdraw.as_str());
    let app = app(Some(ctx.database_url())).await;

    let withdraw_response = app.oneshot(
        Request::builder()
            .uri("/api/storage/300/withdraw")
            .method("PATCH")
            .body(Body::empty())
            .unwrap()
    ).await.unwrap();

    let (parts, body) = withdraw_response.into_parts();

    assert!(&parts.status.is_server_error(), "Expected an internal server error to be returned");

    let bytes = hyper::body::to_bytes(body).await.unwrap();
    let err_msg = std::str::from_utf8(&bytes[..]).unwrap();

    assert_eq!(err_msg, "Storage id not found, update failed");
}

#[tokio::test]
async fn re_enter_updates_storage_correctly() {
    let ctx = Context::new(Mod::Withdraw.as_str());
    let mut app = app(Some(ctx.database_url())).await;

    let withdraw_response = ServiceExt::ready(&mut app)
        .await.unwrap()
        .call(
            Request::builder()
                .uri("/api/storage/5/withdraw")
                .method("PATCH")
                .body(Body::empty())
                .unwrap()
        ).await.unwrap();

    assert!(withdraw_response.status().is_success(), "Withdraw step failed");

    let re_enter_response = ServiceExt::ready(&mut app)
        .await.unwrap()
        .call(
            Request::builder()
                .uri("/api/storage/5/re-enter")
                .method("PATCH")
                .body(Body::empty())
                .unwrap()
        ).await.unwrap();

    let (parts, body) = re_enter_response.into_parts();
    let _bytes = hyper::body::to_bytes(body).await.unwrap();

    assert!(parts.status.is_success(), "Re-enter was not successful");
}

#[tokio::test]
async fn re_enter_storage_returns_error_when_not_found() {
    let ctx = Context::new(Mod::Update.as_str());
    let app = app(Some(ctx.database_url())).await;

    let re_enter_response = app.oneshot(
        Request::builder()
            .uri("/api/storage/300/re-enter")
            .method("PATCH")
            .body(Body::empty())
            .unwrap()
    ).await.unwrap();

    let (parts, body) = re_enter_response.into_parts();

    assert!(parts.status.is_server_error(), "Out of range id did not return error");

    let bytes = hyper::body::to_bytes(body).await.unwrap();
    let err_msg = std::str::from_utf8(&bytes[..]).unwrap();

    assert_eq!(err_msg, "Storage id not found, update failed");
}

#[tokio::test]
async fn delete_storage_works_correcty() {
    let ctx = Context::new(Mod::Delete.as_str());
    let mut app = app(Some(ctx.database_url())).await;

    let delete_response = ServiceExt::ready(&mut app)
        .await.unwrap()
        .call(
            Request::builder()
                .uri("/api/storage/5")
                .method("DELETE")
                .body(Body::empty())
                .unwrap()
        ).await.unwrap();

    assert!(delete_response.status().is_success(), "Delete was not successful");

    let check_response = ServiceExt::ready(&mut app)
        .await.unwrap()
        .call(
            Request::builder()
                .uri("/api/storage/5")
                .body(Body::empty())
                .unwrap()
        ).await.unwrap();

    let (parts, _body) = check_response.into_parts();

    assert!(parts.status.is_server_error(), "Check of deleted id did not return error");

    // let bytes = hyper::body::to_bytes(body).await.unwrap();
    // let err_msg = std::str::from_utf8(&bytes[..]).unwrap();

    // For later. Did not implement a handler to check for the NotFound error from Pg.
    // assert_eq!(err_msg, "Storage id not found, delete failed")
}

#[tokio::test]
async fn delete_storage_returns_error_when_not_found() {
    let ctx = Context::new(Mod::Delete.as_str());
    let app = app(Some(ctx.database_url())).await;

    let delete_response = app.oneshot(
        Request::builder()
            .uri("/api/storage/300")
            .method("DELETE")
            .body(Body::empty())
            .unwrap()
    ).await.unwrap();

    assert!(delete_response.status().is_server_error(), "Delete on wrong id did not return error");
}

mod storage_filters {
    use super::*;

    // Only test to check filter validation. If this works, StorageFilter.parse() is called correctly.
    // Other checks are already tested by unit testing.
    #[tokio::test]
    async fn only_drawer_name_returns_bad_request() {
        let ctx = Context::new(Mod::Filter.as_str());
        let app = app(Some(ctx.database_url())).await;

        let response = app.oneshot(
            Request::builder()
                .uri("/api/storage?drawerName=Schuif%201")
                .body(Body::empty())
                .unwrap()
        ).await.unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);

        let bytes = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let err_msg = std::str::from_utf8(&bytes[..]).unwrap();

        assert_eq!(err_msg, "drawerName also requires freezerName as query parameters");
    }

    #[tokio::test]
    async fn products_returns_correct_vec() {
        let ctx = Context::new(Mod::Filter.as_str());
        let app = app(Some(ctx.database_url())).await;

        let product = Product::from_tuple(PRODUCTS[3]);
        let expected_storage_vec = storage_response_from_storage_vec(
            Storage::from_vec(STORAGE.to_vec()).into_iter().filter(|storage| {
                storage.available
            }).collect::<Vec<Storage>>()
        ).into_iter().filter(|storage| {
            storage.product_name.eq(&product.name)
        }).collect::<Vec<StorageResponse>>();

        let response = app.oneshot(
            Request::builder()
                .uri(format!("/api/storage?productName={}", product.name).replace(' ', "%20").as_str())
                .body(Body::empty())
                .unwrap()
        ).await.unwrap();

        assert!(response.status().is_success(), "productName filter request was not successful");

        let bytes = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let response_vec = serde_json::from_slice::<Vec<StorageResponse>>(&bytes).unwrap();

        assert_eq!(response_vec, expected_storage_vec);
    }

    #[tokio::test]
    async fn drawer_freezer_name_returns_correct_vec() {
        let ctx = Context::new(Mod::Filter.as_str());
        let app = app(Some(ctx.database_url())).await;

        let drawer = Drawer::from_tuple(DRAWERS[10]);
        let freezer = &Freezer::from_vec(FREEZERS.to_vec()).into_iter().filter(|freezer| {
            freezer.freezer_id.eq(&drawer.freezer_id)
        }).collect::<Vec<Freezer>>()[0];
        let expected_storage_vec = storage_response_from_storage_vec(
            Storage::from_vec(STORAGE.to_vec()).into_iter().filter(|storage| {
                storage.available
            }).collect::<Vec<Storage>>()
        ).into_iter().filter(|storage| {
            storage.drawer_name.eq(&drawer.name) && storage.freezer_name.eq(&freezer.name)
        }).collect::<Vec<StorageResponse>>();

        let response = app.oneshot(
            Request::builder()
                .uri(format!("/api/storage?drawerName={}&freezerName={}", drawer.name, freezer.name).replace(' ', "%20").as_str())
                .body(Body::empty())
                .unwrap()
        ).await.unwrap();

        assert!(response.status().is_success(), "drawerName and freezerName filter request was not successful");

        let bytes = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let response_vec = serde_json::from_slice::<Vec<StorageResponse>>(&bytes).unwrap();

        assert_eq!(response_vec, expected_storage_vec);
    }

    #[tokio::test]
    async fn freezer_returns_correct_vec() {
        let ctx = Context::new(Mod::Filter.as_str());
        let app = app(Some(ctx.database_url())).await;

        let freezer = Freezer::from_tuple(FREEZERS[0]);
        let expected_storage_vec = storage_response_from_storage_vec(
            Storage::from_vec(STORAGE.to_vec()).into_iter().filter(|storage| {
                storage.available
            }).collect::<Vec<Storage>>()
        ).into_iter().filter(|storage| {
            storage.freezer_name.eq(&freezer.name)
        }).collect::<Vec<StorageResponse>>();

        let response = app.oneshot(
            Request::builder()
                .uri(format!("/api/storage?freezerName={}", freezer.name).replace(' ', "%20").as_str())
                .body(Body::empty())
                .unwrap()
        ).await.unwrap();

        assert!(response.status().is_success(), "freezerName filter request was not successful");

        let bytes = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let response_vec = serde_json::from_slice::<Vec<StorageResponse>>(&bytes).unwrap();

        assert_eq!(response_vec, expected_storage_vec);
    }


    #[tokio::test]
    async fn in_before_returns_correct_vec() {
        let ctx = Context::new(Mod::Filter.as_str());
        let app = app(Some(ctx.database_url())).await;

        let ref_storage = Storage::from_tuple(STORAGE[24]);
        let expected_storage_vec = storage_response_from_storage_vec(
            Storage::from_vec(STORAGE.to_vec()).into_iter().filter(|storage| {
                storage.available && storage.date_in.lt(&ref_storage.date_in)
            }).collect::<Vec<Storage>>()
        );

        let response = app.oneshot(
            Request::builder()
                .uri(format!("/api/storage?inBefore={}", ref_storage.date_in).replace(' ', "%20").as_str())
                .body(Body::empty())
                .unwrap()
        ).await.unwrap();

        assert!(response.status().is_success(), "beforeIn filter request was not successful");

        let bytes = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let response_vec = serde_json::from_slice::<Vec<StorageResponse>>(&bytes).unwrap();

        assert_eq!(response_vec, expected_storage_vec);
    }

    #[tokio::test]
    async fn expires_after_date_returns_correct_vec() {
        let ctx = Context::new(Mod::Filter.as_str());
        let app = app(Some(ctx.database_url())).await;

        // Sample storage based expiration date to make checking the result easier.
        let ref_storage = Storage::from_tuple(STORAGE[0]);
        let ref_product = &Product::from_vec(PRODUCTS.to_vec())
            .into_iter().filter(|product| {
            product.product_id.eq(&ref_storage.product_id)
        }).collect::<Vec<Product>>()[0];
        let expiration_date = ref_storage.date_in.checked_add_months(Months::new(ref_product.expiration_months as u32)).unwrap();

        let expected_storage_vec = storage_response_from_storage_vec(
            Storage::from_vec(STORAGE.to_vec()).into_iter().filter(|storage| {
                storage.available
            }).collect::<Vec<Storage>>()
        ).into_iter().filter(|storage| {
            storage.expiration_date.ge(&expiration_date)
        }).collect::<Vec<StorageResponse>>();

        let response = app.oneshot(
            Request::builder()
                .uri(format!("/api/storage?expiresAfterDate={}", expiration_date).replace(' ', "%20").as_str())
                .body(Body::empty())
                .unwrap()
        ).await.unwrap();

        assert!(response.status().is_success(), "expiresAfterDate filter request was not successful");

        let bytes = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let response_vec = serde_json::from_slice::<Vec<StorageResponse>>(&bytes).unwrap();

        assert_eq!(response_vec, expected_storage_vec);
    }
    #[tokio::test]
    async fn expires_before_date_returns_correct_vec() {
        let ctx = Context::new(Mod::Filter.as_str());
        let app = app(Some(ctx.database_url())).await;

        // Sample storage based expiration date to make checking the result easier.
        let ref_storage = Storage::from_tuple(STORAGE[10]);
        let ref_product = &Product::from_vec(PRODUCTS.to_vec())
            .into_iter().filter(|product| {
            product.product_id.eq(&ref_storage.product_id)
        }).collect::<Vec<Product>>()[0];
        let expiration_date = ref_storage.date_in.checked_add_months(Months::new(ref_product.expiration_months as u32)).unwrap();

        let expected_storage_vec = storage_response_from_storage_vec(
            Storage::from_vec(STORAGE.to_vec()).into_iter().filter(|storage| {
                storage.available
            }).collect::<Vec<Storage>>()
        ).into_iter().filter(|storage| {
            storage.expiration_date.le(&expiration_date)
        }).collect::<Vec<StorageResponse>>();

        let response = app.oneshot(
            Request::builder()
                .uri(format!("/api/storage?expiresBeforeDate={}", expiration_date).replace(' ', "%20").as_str())
                .body(Body::empty())
                .unwrap()
        ).await.unwrap();

        assert!(response.status().is_success(), "expiresBeforeDate filter request was not successful");

        let bytes = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let response_vec = serde_json::from_slice::<Vec<StorageResponse>>(&bytes).unwrap();

        assert_eq!(response_vec, expected_storage_vec);
    }


    // Unsure how to test this one properly. Would require passing a fixed date as current time to the test.
    // #[tokio::test]
    // async fn expires_in_days_returns_correct_vec() {
    //
    // }

    #[tokio::test]
    async fn not_available_returns_correct_vec() {
        // Only not available as the default get_storage query only returns the available storage
        // items.
        let ctx = Context::new(Mod::Filter.as_str());
        let app = app(Some(ctx.database_url())).await;

        let expected_vec = storage_response_from_storage_vec(
            Storage::from_vec(STORAGE.to_vec()).into_iter().filter(| storage | {
                !storage.available
            }).collect::<Vec<Storage>>()
        );

        let response = app.oneshot(
            Request::builder()
                .uri("/api/storage?available=false")
                .body(Body::empty())
                .unwrap()
        ).await.unwrap();

        assert!(response.status().is_success(), "available filter request was not successful");

        let bytes = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let result_vec = serde_json::from_slice::<Vec<StorageResponse>>(&bytes).unwrap();

        assert_eq!(result_vec, expected_vec);
    }

    #[tokio::test]
    async fn is_withdrawn_returns_correct_vec() {
        let ctx = Context::new(Mod::Filter.as_str());
        let app = app(Some(ctx.database_url())).await;

        let expected_vec = storage_response_from_storage_vec(
            Storage::from_vec(STORAGE.to_vec()).into_iter().filter(| storage | {
                storage.date_out.is_some()
            }).collect::<Vec<Storage>>()
        );

        let response = app.oneshot(
            Request::builder()
                .uri("/api/storage?isWithdrawn=true")
                .body(Body::empty())
                .unwrap()
        ).await.unwrap();

        assert!(response.status().is_success(), "isWithdrawn filter request was not successful");

        let bytes = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let result_vec = serde_json::from_slice::<Vec<StorageResponse>>(&bytes).unwrap();

        assert_eq!(result_vec, expected_vec);
    }

    #[tokio::test]
    async fn min_weight_returns_correct_vec() {
        let ctx = Context::new(Mod::Filter.as_str());
        let app = app(Some(ctx.database_url())).await;

        let expected_vec = storage_response_from_storage_vec(
            Storage::from_vec(STORAGE.to_vec()).into_iter().filter(| storage | {
                storage.weight_grams >= 500.0 && storage.date_out.is_none()
            }).collect::<Vec<Storage>>()
        );

        let response = app.oneshot(
            Request::builder()
                .uri("/api/storage?minWeight=500.0")
                .body(Body::empty())
                .unwrap()
        ).await.unwrap();

        assert!(response.status().is_success(), "minWeight filter request was not successful");

        let bytes = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let result_vec = serde_json::from_slice::<Vec<StorageResponse>>(&bytes).unwrap();

        assert_eq!(result_vec, expected_vec);
    }

    #[tokio::test]
    async fn max_weight_returns_correct_vec() {
        let ctx = Context::new(Mod::Filter.as_str());
        let app = app(Some(ctx.database_url())).await;

        let expected_vec = storage_response_from_storage_vec(
            Storage::from_vec(STORAGE.to_vec()).into_iter().filter(| storage | {
                storage.weight_grams <= 400.0
            }).collect::<Vec<Storage>>()
        );

        let response = app.oneshot(
            Request::builder()
                .uri("/api/storage?maxWeight=400.0")
                .body(Body::empty())
                .unwrap()
        ).await.unwrap();

        assert!(response.status().is_success(), "maxWeight filter request was not successful");

        let bytes = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let result_vec = serde_json::from_slice::<Vec<StorageResponse>>(&bytes).unwrap();

        assert_eq!(result_vec, expected_vec);
    }
}
