//! Endpoint `/api/freezers`, implements `GET`, `POST`, `PATCH`, `DELETE`.
use crate::core::auth::extract_headers_uuid;
use crate::{
    core::{connection::establish_connection, error::internal_error},
    models::{Freezer, NewFreezer},
    router::AppState,
};
use axum::http::HeaderMap;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};
use diesel::prelude::*;
use diesel::QueryDsl;
use tracing::debug;
use std::ops::Deref;

#[axum::debug_handler]
/// Get all freezer entries: `GET /api/freezers`.
///
/// # Returns
///
/// Vec<[Freezer]>, in format `application/json`
pub async fn get_all_freezers(
    headers: HeaderMap,
    State(state): State<AppState>,
    req: axum::http::Request<axum::body::Body>,
) -> Result<Json<Vec<Freezer>>, (StatusCode, String)> {
    debug!("{:?}", headers);
    debug!("{:?}", req);
    let uuid = extract_headers_uuid(headers, &state).map_err(internal_error)?;

    use crate::schema::freezers::dsl::*;

    let conn = &mut establish_connection(state.db_url);
    let result = freezers
        .filter(user_id.eq(uuid))
        .get_results(conn)
        .map_err(internal_error)?;

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
    headers: HeaderMap,
    State(state): State<AppState>,
    Path(request_id): Path<i32>,
) -> Result<Json<Freezer>, (StatusCode, String)> {
    let uuid = extract_headers_uuid(headers, &state).map_err(internal_error)?;

    use crate::schema::freezers::dsl::*;

    let conn = &mut establish_connection(state.db_url);

    let result = freezers
        .filter(user_id.eq(uuid))
        .filter(id.eq(request_id))
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
    headers: HeaderMap,
    State(state): State<AppState>,
    Path(query_name): Path<String>,
) -> Result<Json<Freezer>, (StatusCode, String)> {
    let uuid = extract_headers_uuid(headers, &state).map_err(internal_error)?;

    use crate::schema::freezers::dsl::*;

    let conn = &mut establish_connection(state.db_url);

    let result = freezers
        .filter(user_id.eq(uuid))
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
    headers: HeaderMap,
    State(state): State<AppState>,
    updated_freezer: Json<Freezer>,
) -> Result<Json<Freezer>, (StatusCode, String)> {
    let uuid = extract_headers_uuid(headers, &state).map_err(internal_error)?;

    use crate::schema::freezers::dsl::*;

    let conn = &mut establish_connection(state.db_url);
    let updated_freezer = updated_freezer.deref().to_owned();

    let name_lookup = freezers
        .filter(user_id.eq(uuid))
        .filter(id.ne(&updated_freezer.id))
        .filter(name.eq(&updated_freezer.name))
        .get_results::<Freezer>(conn)
        .map_err(internal_error)?;

    if !name_lookup.is_empty() {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            String::from("This freezer name already exists"),
        ));
    }

    let update_result = diesel::update(freezers)
        .filter(user_id.eq(uuid))
        .filter(id.eq(&updated_freezer.id))
        .set(&updated_freezer)
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
    headers: HeaderMap,
    State(state): State<AppState>,
    new_freezer: Json<NewFreezer>,
) -> Result<Json<Freezer>, (StatusCode, String)> {
    let uuid = extract_headers_uuid(headers, &state).map_err(internal_error)?;

    use crate::schema::freezers::dsl::*;
    let conn = &mut establish_connection(state.db_url);
    let new_freezer = new_freezer.deref().to_owned();

    let name_query = freezers
        .filter(user_id.eq(uuid))
        .filter(name.eq(&new_freezer.name))
        .get_results::<Freezer>(conn)
        .map_err(internal_error)?;

    if !name_query.is_empty() {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            String::from("This freezer name already exists"),
        ));
    }

    if new_freezer.user_id != uuid {
        return Err((
            StatusCode::FORBIDDEN,
            String::from("Authenticated User ID does not match user_id of freezer."),
        ));
    }

    let create_result = diesel::insert_into(freezers)
        .values(new_freezer)
        .returning(Freezer::as_returning())
        .get_result(conn)
        .map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Error while inserting freezer: {}", err),
            );
        })
        .unwrap();

    Ok(Json(create_result))
}

/// Deletes a freezer entry: `DELETE /api/freezers/id=<i32>`.
///
/// # Errors
///
/// * `NotFound`: freezer id not found.
pub async fn delete_freezer(
    headers: HeaderMap,
    State(state): State<AppState>,
    Path(request_id): Path<i32>,
) -> Result<Json<i32>, (StatusCode, String)> {
    let uuid = extract_headers_uuid(headers, &state).map_err(internal_error)?;

    use crate::schema::freezers::dsl::*;
    let conn = &mut establish_connection(state.db_url);

    let id_query = freezers
        .filter(user_id.eq(uuid))
        .find(request_id)
        .get_results::<Freezer>(conn)
        .map_err(internal_error)?;

    if id_query.is_empty() {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            String::from("This freezer id does not exist"),
        ));
    }

    diesel::delete(freezers)
        .filter(user_id.eq(uuid))
        .filter(id.eq(request_id))
        .execute(conn)
        .map_err(internal_error)?;

    Ok(Json(request_id))
}
