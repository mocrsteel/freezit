//! Endpoint `/api/storage`, implements `GET`, `POST`, `PATCH`, `DELETE`.
//!
//! ## Default query returns
//!
//! Any return by querying the /api/storage with or without additional filters will contain the following date, but formatted as Json and following [StorageResponse]:
//!```bash
//! storageId | productName | freezerName | drawerName | weightGrams | expirationDate | expiresInDays   | inStorageSince
//! ----------|-------------|-------------|------------|-------------|----------------|-----------------|---------------
//! 1         | Brocoli     | Garage      | Schuif 1   | 525.3       | 2024-08-01     | 7               | 2023-08-01
//!```
//! Which will give the following Json data:
//! ```json
//! {
//!     "storageId": 1,
//!     "productName": "Brocoli",
//!     "freezerName": "Garage",
//!     "drawerName": "Schuif 1",
//!     "weightGrams": 525.3,
//!     "expirationDate": "2024-08-01",
//!     "expiresInDays": 128,
//!     "inStorageSince": "2023-08-01",
//!     "outStorageSince: "2024-08-01",
//! }
//! ```
//! ## Possible queries
//!
//! Query by:
//!
//! * storage_id
//! * storage in general, but filtered on possible filters given in [StorageFilter]. All are to be defined in a query parameter: `/api/storage?productName=Brocoli`.
//!
use std::fmt::Debug;
use std::ops::Deref;
use std::sync::Arc;
use axum::extract::{Path, Query, State};
use axum::Json;
use chrono::{NaiveDate, Local};
use diesel::prelude::*;
use hyper::StatusCode;
use serde::{Deserialize, Serialize};
use struct_iterable::Iterable;

use crate::{AppState, schema};
use crate::core::connection::establish_connection;
use crate::core::error::internal_error;
use crate::core::query::{empty_string_as_none, ExpirationData};
use crate::models::*;
use crate::schema::freezers::dsl as freezers_dsl;
use crate::schema::drawers::dsl as drawers_dsl;
use crate::schema::products::dsl as products_dsl;

/// Struct containing the possible query parameters to query the storage table of the database.
/// As the complexity of these  queries can increase pretty fast, some handlers and checks are built to parse the
/// request properly and ensure it is valid.
///
/// # Input from frontend
///
/// All parameters are deserialized from camelCase and should be entered as such from the frontend.
#[derive(Debug, Deserialize, Iterable)]
#[serde(rename_all = "camelCase")]
pub struct StorageFilter {
    /// Name of the product to be queried, will return all products matching it.
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub product_name: Option<String>,
    /// ID of the drawer that is selected. Can be combined with product_id, freezer_id, freezer_name, drawer_name,
    /// in_before, expires_in_months, expires_after_date, available, min_weight and max_weight.
    // pub drawer_id: Option<i32>,
    // /// Name of the drawer to be queried. Must be accompanied with the freezer_id or freezer_name.
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub drawer_name: Option<String>,
    // /// ID of the freezer where the storage should be queried.
    // pub freezer_id: Option<i32>,
    /// Name of the freezer to be queried.
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub freezer_name: Option<String>,
    /// Date in format [RFC3339](https://datatracker.ietf.org/doc/html/rfc3339): `<YYYY>-<MM>-<DD>T<HH>:<MM>:<SS>Z`
    /// (Z because the date is parsed to local!). E.g.: `2023-12-23T13:00:00Z`.
    pub in_before: Option<NaiveDate>,
    /// Days until which the product expires (and products that expire sooner)
    pub expires_in_days: Option<i32>,
    /// Date after which products expire, in format RFC3339: `<YYYY>-<MM>-<DD>T<HH>:<MM>:<SS>Z`, e.g. `2023-12-23T13:00:00Z`.
    pub expires_after_date: Option<NaiveDate>,
    /// Date before which products expire, in format RFC3339: `<YYYY>-<MM>-<DD>T<HH>:<MM>:<SS>Z`, e.g. `2023-12-23T13:00:00Z`.
    pub expires_before_date: Option<NaiveDate>,
    /// Selects all products that have a date_out specified (i.e. are withdrawn from the freezer). Defaults to false.
    #[serde(default = "is_withdrawn_default")]
    pub is_withdrawn: Option<bool>,
    /// Minimum weight to be queried, defaults to 0.0 grams.
    #[serde(default = "weight_min_default")]
    pub min_weight: Option<f32>,
    /// Maximum weight to be queries in grams, defaults to 10000.0 grams.
    #[serde(default = "weight_max_default")]
    pub max_weight: Option<f32>,
}

impl StorageFilter {
    /// Checks if the query meets constraints to be respected. See [get_storage] docs for the constraints in place.
    pub fn parse(&self) -> Result<(), (StatusCode, String)> {
        if self.drawer_name.is_some() && self.freezer_name.is_none() {
            return Err((StatusCode::BAD_REQUEST, String::from("drawerName also requires freezerName as query parameters")));
        }
        // Obsolete if we keep freezer_id out of the filter parameters.
        // if self.freezer_name.is_some() && self.freezer_id.is_some() {
        //     return Err((StatusCode::BAD_REQUEST, String::from("Querying freezerName and freezerId at the same time is not allowed")))
        // }
        if self.in_before.is_some() && self.expires_after_date.is_some() {
            let date_in = self.in_before.unwrap();
            let date_expires = self.expires_after_date.unwrap();

            if date_in >= date_expires {
                return Err((StatusCode::BAD_REQUEST, String::from("inBefore cannot be later than expiresAfterDate")));
            }
        }
        if self.expires_before_date.is_some() && self.expires_after_date.is_some() {
            let before = self.expires_before_date.unwrap();
            let after = self.expires_after_date.unwrap();

            if before <= after {
                return Err((StatusCode::BAD_REQUEST, String::from("expiresBeforeDate canot be equal or earlier than expiresAfterDate")));
            }
        }
        let min_weight = self.min_weight.unwrap();
        let max_weight = self.max_weight.unwrap();
        if min_weight >= max_weight {
            return Err((StatusCode::BAD_REQUEST, String::from("minWeight must be smaller than maxWeight")))
        }

        Ok(())
    }
}

/// Struct representing the returned object when querying the storage endpoint.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StorageResponse {
    /// ID of the storage item.
    pub storage_id: i32,
    /// Name of the product linked to the [Storage] `product_id`.
    pub product_name: String,
    /// Name of the freezer linked to the [Storage] `drawer_id`.
    pub freezer_name: String,
    /// Name of the drawer linked to the [Storage] `drawer_id`.
    pub drawer_name: String,
    /// Weight of the storage item, in grams.
    pub weight_grams: f32,
    /// Time until the storage item expires, expressed in days.
    /// Calculated from the [Storage] `date_in`, [Product] `expiration_months` and [Local] `now` time.
    pub expires_in_days: i64,
    /// Date of expiration of the storage item, calculated from [Storage] date of entry and [Product] expiration time.
    pub expiration_date: NaiveDate,
    /// Date of entry, renamed for frontend readability: [Storage] `date_in`.
    pub in_storage_since: NaiveDate,
    /// Date of withdrawal. Is `None` when still in storage.
    pub out_storage_since: Option<NaiveDate>,
}

impl StorageResponse {
    /// Turns inner join query on all tables into a common [StorageResponse] to be consumed by the frontend.
    pub fn from_query_result(query_result: Vec<(Storage, Product, Drawer, Freezer)>) -> Vec<Self> {
        query_result
            .into_iter()
            .map(|(stor, prod, draw, freez)| {
                let expiration_data = ExpirationData::new(stor.date_in, prod.expiration_months);
                StorageResponse {
                    storage_id: stor.storage_id,
                    product_name: prod.name,
                    freezer_name: freez.name,
                    drawer_name: draw.name,
                    weight_grams: stor.weight_grams,
                    expires_in_days: expiration_data.expires_in_days,
                    expiration_date: expiration_data.date_expires,
                    in_storage_since: stor.date_in,
                    out_storage_since: stor.date_out,
                }
            })
            .collect::<Vec<StorageResponse>>()
    }
}
impl PartialEq for StorageResponse {
    fn eq(&self, other: &Self) -> bool {
        self.storage_id == other.storage_id
        && self.product_name == other.product_name
        && self.freezer_name == other.freezer_name
        && self.drawer_name == other.drawer_name
        && (self.weight_grams - other.weight_grams).abs() <= 1e-6
        && self.expires_in_days == other.expires_in_days
        && self.expiration_date == other.expiration_date
        && self.in_storage_since == other.in_storage_since
        && self.out_storage_since == other.out_storage_since
    }
}

fn weight_min_default() -> Option<f32> {
    Some(0.0)
}

fn weight_max_default() -> Option<f32> {
    Some(1000.0)
}

fn is_withdrawn_default() -> Option<bool> {
    Some(false)
}

/// Get all storage entries: `GET /api/storage`.
///
/// # Accepted query parameters for filtering
///
/// * `productName=<String>`: Name of the product.
/// * `freezerName=<String>`: Name of the freezer.
/// * `drawerName=<String>`: Name of the drawer.
/// * `inBefore=<DateTime String>`: Products that have been put in storage before this date.
/// * `expiresInMonths=<i32>`: Time until or before which products expire.
/// * `expiresAfterDate=<DateTime String>`: Date after which products expire.
/// * `expiresBeforeDate=<DateTime String>`: Date before which products expire.
/// * `isWithdrawn=<bool>` **(defaults to false)**: Product has been withdrawn or not.
/// * `minWeight=<f32>` **(defaults to 0.0)**: Minimum product weight filter, in grams.
/// * `maxWeight=<f32>` **(defaults to 100000.0)**: Maximum product weight filter, in grams.
///
/// # Query parameter constraints
///
/// * `drawerName` must be used in tandem with `freezerName`.
///
/// # Returns
///
/// Vec<[Storage]>
pub async fn get_storage(State(state): State<AppState>, params: Query<StorageFilter>) -> Result<Json<Vec<StorageResponse>>, (StatusCode, String)> {
    params.parse()?;
    use schema::storage::dsl::*;

    let conn = &mut establish_connection(state.db_url);

    let mut query = storage
        .inner_join(products_dsl::products)
        .inner_join(drawers_dsl::drawers)
        .inner_join(freezers_dsl::freezers.on(freezers_dsl::freezer_id.eq(drawers_dsl::freezer_id)))
        .into_boxed();

    if params.product_name.is_some() {
        query = query.filter(products_dsl::name.eq(params.product_name.as_ref().unwrap()));
    }
    if params.freezer_name.is_some() {
        query = query.filter(freezers_dsl::name.eq(params.freezer_name.as_ref().unwrap()));

        if params.drawer_name.is_some() {
            query = query.filter(drawers_dsl::name.eq(params.drawer_name.as_ref().unwrap()));
        }
    }
    if params.in_before.is_some() {
        let date_max_naive = params.in_before.unwrap();
        query = query.filter(date_in.lt(date_max_naive))
    }

    // Filters defined with default parameters.

    // Withdrawn means it's taken out -> Date_out is no longer NULL. If default (false), then we only
    // want rows where date_out is NULL.
    if params.is_withdrawn.unwrap() {
        query = query.filter(date_out.is_not_null());
    } else {
        query = query.filter(date_out.is_null())
    }

    query = query
        .filter(weight_grams.le(params.max_weight.as_ref().unwrap()))
        .filter(weight_grams.ge(params.min_weight.as_ref().unwrap()));

    let storage_results = query
        .select((Storage::as_select(), Product::as_select(), Drawer::as_select(), Freezer::as_select()))
        .order_by(storage_id)
        .load::<(Storage, Product, Drawer, Freezer)>(conn)
        // .get_results::<Storage>(conn)
        .map_err(internal_error)?;
    let zipped_result = StorageResponse::from_query_result(storage_results);

    // Expiration filters, calculated after search in database.
    let zipped_result = match params.expires_in_days {
        Some(days) => {
            zipped_result
                .into_iter()
                .filter(|data| {
                    data.expires_in_days <= days.into()
                })
                .collect::<Vec<StorageResponse>>()
        }
        None => zipped_result
    };
    let zipped_result = match params.expires_after_date {
        Some(date) => {
            let max_expiration_date: Arc<NaiveDate> = Arc::new(date);
            zipped_result
                .into_iter()
                .filter(move |data| {
                    let date = Arc::clone(&max_expiration_date);
                    data.expiration_date >= *date
                }).collect::<Vec<StorageResponse>>()
        }
        None => zipped_result,
    };
    let zipped_result = match params.expires_before_date {
        Some(date) => {
            let date_before: Arc<NaiveDate> = Arc::new(date);
            zipped_result
                .into_iter()
                .filter(move |data| {
                    let date = Arc::clone(&date_before);
                    data.expiration_date <= *date
                })
                .collect::<Vec<StorageResponse>>()
        }
        None => zipped_result,
    };


    Ok(Json(zipped_result))
}

/// Get a storage entry by its id: `GET /api/storage/<i32>`.
///
/// # Returns
///
/// [Storage]
pub async fn get_storage_by_id(State(state): State<AppState>, Path(id): Path<i32>) -> Result<Json<Vec<StorageResponse>>, (StatusCode, String)> {
    use crate::schema::storage::dsl::*;

    let conn = &mut establish_connection(state.db_url);

    let storage_results = storage
        .inner_join(products_dsl::products) // .on(products_dsl::product_id.eq(product_id))
        .inner_join(drawers_dsl::drawers) // .on(drawers_dsl::drawer_id.eq(drawer_id))
        .inner_join(freezers_dsl::freezers.on(freezers_dsl::freezer_id.eq(drawers_dsl::freezer_id)))
        .filter(storage_id.eq(id))
        .select((Storage::as_select(), Product::as_select(), Drawer::as_select(), Freezer::as_select()))
        .load::<(Storage, Product, Drawer, Freezer)>(conn)
        .map_err(internal_error)?;

    if storage_results.is_empty() {
        return Err((StatusCode::INTERNAL_SERVER_ERROR, String::from("Storage item not found")));
    }

    let result = StorageResponse::from_query_result(storage_results);

    Ok(Json(result))
}

/// Create a new storage entry: `POST /api/storage`.
///
/// # Required body
///
/// [NewStorageItem]
///
/// # Returns
///
/// The ID of the newly created storage item.
///
/// # Errors
///
/// * Can't have a duplicate error on this one.
pub async fn create_storage(State(state): State<AppState>, new_storage_item: Json<NewStorageItem>) -> Result<Json<Vec<StorageResponse>>, (StatusCode, String)> {
    use crate::schema::storage::dsl::*;

    let conn = &mut establish_connection(state.db_url.clone());
    let new_storage_item = new_storage_item.deref();
    let insert_result = diesel::insert_into(storage)
        .values(new_storage_item)
        .returning(storage_id)
        .get_results::<i32>(conn)
        .map_err(internal_error)?;

    get_storage_by_id(State(state), Path(insert_result[0])).await
}

/// Update an existing storage entry: `PATCH /api/storage`.
///
/// # Required body
///
/// [Storage]: updated storage item.
/// Storage ID should not be changed and should be unique.
///
/// # Returns
///
/// [Storage] that was just updated, in format `application/json`
///
/// # Errors
///
/// * `DuplicateError`: Storage ID already taken, usually caused by a database error.
pub async fn update_storage(State(state): State<AppState>, updated_storage_frontend: Json<StorageResponse>) -> Result<Json<Vec<StorageResponse>>, (StatusCode, String)>{
    use crate::schema::storage::dsl::*;

    let conn = &mut establish_connection(state.db_url.clone());

    let storage_entry = storage
        .filter(storage_id.eq(&updated_storage_frontend.storage_id))
        .select(Storage::as_select())
        .get_results::<Storage>(conn)
        .map_err(internal_error)?;
    if storage_entry.is_empty() {
        return Err((StatusCode::INTERNAL_SERVER_ERROR, String::from("Storage item not found")))
    }
    let storage_entry = &storage_entry[0];
    let product = products_dsl::products
        .filter(products_dsl::name.eq(&updated_storage_frontend.product_name))
        .select(Product::as_select())
        .load::<Product>(conn)
        .map_err(internal_error)?;
    if product.is_empty() {
        return Err((StatusCode::INTERNAL_SERVER_ERROR, String::from("Product name not found")));
    }
    let product = &product[0];
    let drawer = drawers_dsl::drawers
        .inner_join(freezers_dsl::freezers)
        .filter(drawers_dsl::name.eq(&updated_storage_frontend.drawer_name))
        .filter(freezers_dsl::name.eq(&updated_storage_frontend.freezer_name))
        .select((Drawer::as_select(), Freezer::as_select()))
        .load::<(Drawer, Freezer)>(conn)
        .map_err(internal_error)?;
    if drawer.is_empty() {
        return Err((StatusCode::INTERNAL_SERVER_ERROR, String::from("Combination of freezerName and drawerName not found")))
    }
    let (drawer, freezer) = &drawer[0];

    let updated_storage_frontend = updated_storage_frontend.deref();
    let update_storage = Storage {
        storage_id: storage_entry.storage_id,
        product_id: product.product_id,
        drawer_id: drawer.drawer_id,
        weight_grams: updated_storage_frontend.weight_grams,
        date_in: updated_storage_frontend.in_storage_since,
        date_out: storage_entry.date_out,
    };

    let update_result = diesel::update(storage)
        .filter(storage_id.eq(&update_storage.storage_id))
        .set(&update_storage)
        .returning(Storage::as_returning())
        .get_result(conn)
        .map_err(internal_error)?;

    let expiration = ExpirationData::new(update_result.date_in, product.expiration_months);
    let response = StorageResponse {
        storage_id: update_result.storage_id,
        product_name: product.name.clone(),
        freezer_name: freezer.name.clone(),
        drawer_name: drawer.name.clone(),
        weight_grams: update_result.weight_grams,
        in_storage_since: update_result.date_in,
        out_storage_since: update_result.date_out,
        expires_in_days: expiration.expires_in_days,
        expiration_date: expiration.date_expires,
    };

    Ok(Json(vec![response]))
}

/// Used when a product is removed from the storage (consumed/thrown away): `PATCH /api/storage/<i32>/withdraw`.
/// Sets the storage item availability to `false` and sets the withdrawn date to the current date.
/// The storage item is not dropped from the database.
///
/// # Requires
///
/// `storage_id` which does not have an availability set to `false`.
///
/// # Errors
///
/// * `AvailabilityError`: already not available.
/// * `ExpirationError`: storage item has expired.
pub async fn withdraw_storage(State(state): State<AppState>, Path(id): Path<i32>) -> Result<(), (StatusCode, String)> {
    use crate::schema::storage::dsl::*;

    let conn = &mut establish_connection(state.db_url);

    let today = Local::now().date_naive();
    let update_result = diesel::update(storage)
        .filter(storage_id.eq(id))
        .set(date_out.eq(today))
        .load::<Storage>(conn)
        .map_err(internal_error)?;
    if update_result.is_empty() {
        return Err((StatusCode::INTERNAL_SERVER_ERROR, String::from("Storage id not found, update failed")))
    }

    Ok(())
}

/// Used when a product is re-entered in storage (mistakenly taken out): `PATCH /api/storage/<i32>/re-enter`.
/// Sets the storage item availability to `true` and erases the withdrawn date (sets it to None).
///
/// # Requires
///
/// `storage_id` which does not have an availability set to `false`.
///
/// # Errors
///
/// * `AvailabilityError`: already not available.
/// * `ExpirationError`: storage item has expired.
pub async fn re_enter_storage(State(state): State<AppState>, Path(id): Path<i32>) -> Result<(), (StatusCode, String)> {
    use crate::schema::storage::dsl::*;

    let conn = &mut establish_connection(state.db_url);

    let update_result = diesel::update(storage)
        .filter(storage_id.eq(id))
        .set(&UpdateStorageAvailability {
            date_out: None,
        })
        .load::<Storage>(conn)
        .map_err(internal_error)?;
    if update_result.is_empty() {
        return Err((StatusCode::INTERNAL_SERVER_ERROR, String::from("Storage id not found, update failed")))
    }

    Ok(())
}

/// Delete a storage item from the database: `DELETE /api/storage/id=<i32>`.
///
/// # Requires
///
/// A valid `storage_id`.
///
/// # Errors
///
/// * `NotFound`: `storage_id` does not exist.
pub async fn delete_storage(State(state): State<AppState>, Path(id): Path<i32>) -> Result<(), (StatusCode, String)> {
    use crate::schema::storage::dsl::*;

    let conn = &mut establish_connection(state.db_url);

    let id_check = storage
        .filter(storage_id.eq(&id))
        .load::<Storage>(conn)
        .map_err(internal_error)?;
    if id_check.is_empty() {
        return Err((StatusCode::INTERNAL_SERVER_ERROR, String::from("Storage id not found, delete failed")));
    }
    diesel::delete(storage)
        .filter(storage_id.eq(id))
        .execute(conn)
        .map_err(internal_error)?;

    Ok(())
}

#[cfg(test)]
mod storage_filter {
    use super::*;

    mod parse {
        use chrono::Days;

        use super::*;
        #[test]
        fn only_drawer_name_returns_error() {
            let storage_filter = StorageFilter {
                product_name: None,
                drawer_name: Some(String::from("Drawer 1")),
                freezer_name: None,
                in_before: None,
                expires_in_days: None,
                expires_after_date: None,
                expires_before_date: None,
                is_withdrawn: None,
                min_weight: None,
                max_weight: None
            };
            let result = storage_filter.parse();

            assert!(result.is_err(), "Expected an error");
            assert_eq!(result.err(), Some((StatusCode::BAD_REQUEST, String::from("drawerName also requires freezerName as query parameters"))))
        }

        #[test]
        fn in_before_ge_date_expires_returns_error() {
            let today = Local::now().date_naive();
            let yesterday = today.checked_sub_days(Days::new(1)).unwrap();
            let storage_filter = StorageFilter {
                product_name: None,
                drawer_name: None,
                freezer_name: None,
                in_before: Some(today),
                expires_in_days: None,
                expires_after_date: Some(yesterday),
                expires_before_date: None,
                is_withdrawn: None,
                min_weight: None,
                max_weight: None
            };
            let result = storage_filter.parse();

            assert!(result.is_err(), "Expected error");
            assert_eq!(result.err(), Some((StatusCode::BAD_REQUEST, String::from("inBefore cannot be later than expiresAfterDate"))))
        }

        #[test]
        fn expires_before_lt_expires_after_returns_error() {
            let today = Local::now().date_naive();
            let yesterday = today.checked_sub_days(Days::new(1)).unwrap();
            let storage_filter = StorageFilter {
                product_name: None,
                drawer_name: None,
                freezer_name: None,
                in_before: None,
                expires_in_days: None,
                expires_after_date: Some(today),
                expires_before_date: Some(yesterday),
                is_withdrawn: None,
                min_weight: None,
                max_weight: None
            };
            let result = storage_filter.parse();

            assert!(result.is_err(), "Expected error");
            assert_eq!(result.err(), Some((StatusCode::BAD_REQUEST, String::from("expiresBeforeDate canot be equal or earlier than expiresAfterDate"))))
        }
        #[test]
        fn min_weight_gt_max_weight_returns_error() {
            let storage_filter = StorageFilter {
                product_name: None,
                drawer_name: None,
                freezer_name: None,
                in_before: None,
                expires_in_days: None,
                expires_after_date: None,
                expires_before_date: None,
                is_withdrawn: None,
                min_weight: Some(500.),
                max_weight: Some(100.)
            };
            let result = storage_filter.parse();

            assert!(result.is_err(), "Expected error");
            assert_eq!(result.err(), Some((StatusCode::BAD_REQUEST, String::from("minWeight must be smaller than maxWeight"))))
        }
    }
}

#[cfg(test)]
mod storage_response {
    use super::*;

    #[test]
    fn from_query_result_returns_correctly() { 
        let query_result = vec![(
            Storage {
                storage_id: 1,
                product_id: 2,
                drawer_id: 3,
                weight_grams: 4.0,
                date_in: NaiveDate::from_ymd_opt(2023, 1, 1).unwrap(),
                date_out: None,
            },
            Product {
                product_id: 2,
                name: String::from("product name"),
                expiration_months: 12
            },
            Drawer {
                drawer_id: 3,
                name: String::from("drawer name"),
                freezer_id: 4
            },
            Freezer {
                freezer_id: 5,
                name: String::from("freezer name")
            }
        )];
        let storage_response = StorageResponse::from_query_result(query_result);
        let expected_storage_response = StorageResponse {
            storage_id: 1,
            product_name: String::from("product name"),
            freezer_name: String::from("freezer name"),
            drawer_name: String::from("drawer name"),
            weight_grams: 4.0,
            expires_in_days: 0,
            expiration_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            in_storage_since: NaiveDate::from_ymd_opt(2023, 1, 1).unwrap(),
            out_storage_since: None,
        };
        let stor = &storage_response[0];

        assert_eq!(storage_response.len(), 1);
        assert_eq!(stor.storage_id, expected_storage_response.storage_id);
        assert_eq!(stor.product_name, expected_storage_response.product_name);
        assert_eq!(stor.freezer_name, expected_storage_response.freezer_name);
        assert_eq!(stor.drawer_name, expected_storage_response.drawer_name);
        assert_eq!(stor.weight_grams, expected_storage_response.weight_grams);
        assert_eq!(stor.in_storage_since, expected_storage_response.in_storage_since);
        assert_eq!(stor.expiration_date, expected_storage_response.expiration_date);
        assert_eq!(stor.out_storage_since, expected_storage_response.out_storage_since);
    }
}
