//! Endpoint `/api/freezers`, implements `GET`, `POST`, `PATCH`, `DELETE`.

use crate::models::{Freezer, NewFreezer};

/// Get all freezer entries: `GET /api/freezers`.
///
/// # Returns
///
/// Vec<[Freezer]>, in format `application/json`
pub async fn get_all_freezers() -> String { "Get all freezers".to_string() }

/// Get a freezer entry by its id: `GET /api/freezers/id=<i32>`.
///
/// # Returns
///
/// [Freezer], in format `application/json`
///
/// # Errors
///
/// * `NotFound`: Freezer id does not exist.
pub async fn get_freezer_by_id() {}

/// Get a freezer entry by its name: `GET /api/freezers/name=<String>`.
///
/// # Returns
///
/// [Freezer], in format `application/json`
///
/// # Errors
///
/// * `NotFound`: Freezer name does not exist.
pub async fn get_freezer_by_name() {}

/// Update a freezer entry: `PATCH /api/freezers/name=<String>`.
///
/// # Required body
///
/// [Freezer] with a unique name.
///
/// # Returns
///
/// [Freezer], in format `application/json`
///
/// # Errors
///
/// * `NotFound`: Freezer name does not exist.
/// * `DuplicateError`: Freezer name already exists.
pub async fn update_freezer() {}

/// Create a new freezer entry: `POST /api/freezers`.
///
/// # Required body
///
/// [NewFreezer]: Name must be unique.
///
/// # Returns
///
/// [Freezer] that was just created, in format `application/json`
///
/// # Errors
///
/// * `DuplicateError`: freezer name already taken.
pub async fn create_freezer() {}

/// Deletes a freezer entry: `DELETE /api/freezers/id=<i32>`.
///
/// # Errors
///
/// * `NotFound`: freezer id not found.
pub async fn delete_freezer() {}