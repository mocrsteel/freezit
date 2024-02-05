use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use tower::{Service, ServiceExt};

use api::{
    app,
    models::{Storage, NewStorageItem, UpdateStorageAvailability}
};

use crate::common::db::Context;
use crate::common::db_data::{
    STORAGE,
    PRODUCTS,
    DRAWERS,
    FREEZERS,
};

static _MOD: &str = "router_storage";

#[tokio::test]
async fn creates_storage_correctly() {}

#[tokio::test]
async fn updates_storage_correctly() {}

#[tokio::test]
async fn withdraw_updates_storage_correctly() {}

#[tokio::test]
async fn re_enter_updates_storage_correctly() {}

#[tokio::test]
async fn delete_storage_works_correclyt() {}

#[tokio::test]
async fn get_storage_returns_storage_response() {}

#[tokio::test]
async fn get_storage_root_returns_all_storage() {}

mod storage_filters {
    use super::*;

    // Only test to check filter validation. If this works, StorageFilter.parse() is called correctly.
    // Other checks are already tested by unit testing.
    #[tokio::test]
    async fn only_drawer_name_returns_bad_request() {}


}

