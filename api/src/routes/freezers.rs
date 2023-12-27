//! Endpoint `/api/freezers`, implements `GET`, `POST`, `PATCH`, `DELETE`.
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};
use diesel::prelude::*;
use diesel::QueryDsl;
use std::ops::Deref;

use crate::{
    connection::establish_connection,
    error::internal_error,
    models::{Freezer, NewFreezer},
    schema::drawers::freezer_id,
    AppState,
};

/// Get all freezer entries: `GET /api/freezers`.
///
/// # Returns
///
/// Vec<[Freezer]>, in format `application/json`
pub async fn get_all_freezers(
    State(state): State<AppState>,
) -> Result<Json<Vec<Freezer>>, (StatusCode, String)> {
    use crate::schema::freezers::dsl::*;

    let conn = &mut establish_connection(state.db_url);

    let result = freezers.load::<Freezer>(conn).map_err(internal_error)?;

    Ok(Json(result))
}

/// Get a freezer entry by its id: `GET /api/freezers/id=<i32>`.
///
/// # Returns
///
/// [Freezer], in format `application/json`
///
/// # Errors
///
/// * `NotFound`: Freezer id does not exist.
pub async fn get_freezer_by_id(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<Freezer>, (StatusCode, String)> {
    use crate::schema::freezers::dsl::*;

    let conn = &mut establish_connection(state.db_url);

    let result = freezers
        .filter(freezer_id.eq(id))
        .get_result(conn)
        .map_err(internal_error)?;

    Ok(Json(result))
}

/// Get a freezer entry by its name: `GET /api/freezers/name=<String>`.
///
/// # Returns
///
/// [Freezer], in format `application/json`
///
/// # Errors
///
/// * `NotFound`: Freezer name does not exist.
pub async fn get_freezer_by_name(
    State(state): State<AppState>,
    Path(query_name): Path<String>,
) -> Result<Json<Freezer>, (StatusCode, String)> {
    use crate::schema::freezers::dsl::*;

    let conn = &mut establish_connection(state.db_url);

    let result = freezers
        .filter(name.eq(query_name))
        .get_result(conn)
        .map_err(internal_error)?;

    Ok(Json(result))
}

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
pub async fn update_freezer(
    State(state): State<AppState>,
    updated_freezer: Json<Freezer>,
) -> Result<Json<Freezer>, (StatusCode, String)> {
    use crate::schema::freezers::dsl::*;

    let conn = &mut establish_connection(state.db_url);
    let updated_freezer = updated_freezer.deref().to_owned();

    let name_lookup = freezers
        .filter(freezer_id.ne(&updated_freezer.freezer_id))
        .filter(name.eq(&updated_freezer.name))
        .get_results::<Freezer>(conn)
        .map_err(internal_error)?;

    if name_lookup.len() > 0 {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            String::from("This freezer name already exists"),
        ));
    }

    let update_result = diesel::update(freezers)
        .filter(freezer_id.eq(&updated_freezer.freezer_id))
        .set(updated_freezer)
        .returning(Freezer::as_returning())
        .get_result(conn)
        .map_err(internal_error)?;

    Ok(Json(update_result))
}

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
pub async fn create_freezer(
    State(state): State<AppState>,
    new_freezer: Json<NewFreezer>,
) -> Result<Json<Freezer>, (StatusCode, String)> {
    use crate::schema::freezers::dsl::*;
    let conn = &mut establish_connection(state.db_url);
    let new_freezer = new_freezer.deref().to_owned();

    let name_query = freezers
        .filter(name.eq(&new_freezer.name))
        .get_results::<Freezer>(conn)
        .map_err(internal_error)?;

    if !name_query.is_empty() {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            String::from("This freezer name already exists"),
        ));
    }

    let create_result = diesel::insert_into(freezers)
        .values(new_freezer)
        .returning(Freezer::as_returning())
        .get_result(conn)
        .map_err(|err| {
          (StatusCode::INTERNAL_SERVER_ERROR, format!("Error while inserting freezer: {}", err));
        }).unwrap();

    Ok(Json(create_result))
}

/// Deletes a freezer entry: `DELETE /api/freezers/id=<i32>`.
///
/// # Errors
///
/// * `NotFound`: freezer id not found.
pub async fn delete_freezer(State(state): State<AppState>, Path(id): Path<i32>) -> Result<Json<i32>, (StatusCode, String)> {
    use crate::schema::freezers::dsl::*;
    let conn = &mut establish_connection(state.db_url);

    let id_query = freezers
        .find(id)
        .get_results::<Freezer>(conn)
        .map_err(internal_error)?;
    if id_query.is_empty() {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            String::from("This freezer id does not exist"),
        ));
    }

    diesel::delete(freezers)
        .filter(freezer_id.eq(id))
        .execute(conn)
        .map_err(internal_error)?;

    Ok(Json(id))
}
