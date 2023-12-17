//! Endpoint `/api/storage`, implements `GET`, `POST`, `PATCH`, `DELETE`.

use crate::models::{Storage, NewStorageItem};

/// Get all storage entries: `GET /api/storage`.
///
/// # Returns
///
/// Vec<[Storage]>
pub async fn get_all_storage() {}

/// Get a storage entry by its `storage_id`: `GET /api/storage/id=<i32>`.
///
/// # Returns
///
/// [Storage], in format `application/json`
///
/// # Errors
///
/// * `NotFound`
pub async fn get_storage_by_id() {}

/// Get all storage entries in a drawer by its `drawer_id`: `GET /api/storage/drawer?id=<i32>`.
///
/// # Returns
///
/// Vec<[Storage]>, in format `application/json`
///
/// # Errors
///
/// * `NotFound`
pub async fn get_storage_by_drawer_id() {}

/// Get all storage entries in a freezer by its `freezer_id`: `GET /api/storage/freezer?id=<i32>`.
///
/// # Returns
///
/// Vec<[Storage]>, in format `application/json`
///
/// # Errors
///
/// * `NotFound`
pub async fn get_storage_by_freezer_id() {}

/// Get all storage entries by their expiration dates: `GET /api/storage/expiration_months=<i32>`.
///
/// # Returns
///
/// Vec<[Storage]>, in format `application/json`
///
/// # Errors
///
/// * `NotFound`
pub async fn get_storage_order_by_expiration() {}

/// Create a new storage entry: `POST /api/storage`.
///
/// # Required body
///
/// [NewStorageItem]
///
/// # Returns
///
/// [Storage] that was just created, in format `application/json`
///
/// # Errors
///
/// * `DuplicateError`: Storage ID already taken, usually caused by a database error.
pub async fn create_storage() {}

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
pub async fn update_storage() {}

/// Used when a product is removed from the storage (consumed/thrown away): `PATCH /api/storage/id=<i32>`.
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
pub async fn withdraw_storage() {}

/// Delete a storage item from the database: `DELETE /api/storage/id=<i32>`.
///
/// # Requires
///
/// A valid `storage_id`.
///
/// # Errors
///
/// * `NotFound`: `storage_id` does not exist.
pub async fn delete_storage() {}

