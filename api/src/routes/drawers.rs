//! Endpoint `/api/drawers`, implements `GET`, `POST`, `PATCH`, `DELETE`.

use axum::http::HeaderMap;
use axum::{
    extract::{Json, Path, Query, State},
    http::StatusCode,
};
use diesel::prelude::*;
use diesel::{QueryDsl, RunQueryDsl};
use serde::Deserialize;
use std::ops::Deref;

use crate::core::auth::extract_headers_uuid;
use crate::core::{
    connection::establish_connection, error::internal_error, query::empty_string_as_none,
};
use crate::models::{Drawer, NewDrawer};
use crate::router::AppState;

/// Allowed query parameters to `GET` drawers. Any query parameters not in this struct will default to query all drawers.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DrawerQueryOptions {
    /// ID of the drawer.
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub d_id: Option<i32>,
    /// ID of the freezer containing one or more drawers.
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub freezer_id: Option<i32>,
    /// Name of the drawer to be requested.
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub drawer_name: Option<String>,
}

/// Gets drawers, taking the query parameters defined in [DrawerQueryOptions] into account: `GET /api/drawers`.
///
/// Parameters have to be inputted as `camelCase` from the frontend.
///
/// # Accepted query parameters
///
/// * `drawerId=<i32>`, deserializes into `drawer_id`. No other parameters accepted if defined.
/// * `freezerId=<i32>`, deserializes into `freezer_id`.
/// * `drawerName=<String>`, deserializes into `drawer_name`.
/// * `freezerId=<i32>&drawerName=<i32>`: deserializes in the respective `snake_case` names.
///
/// # Returns
///
/// ## Result
///
/// A Vec of [Drawer]'s. Is empty when no matches are found.
///
/// ## Default
///
/// If non-defined query parameters are given, the request to drawers defaults to get all drawers in the database.
///
/// ## Error
///
/// * 400: [StatusCode::BAD_REQUEST] when incorrect combinations of parameters are given.
/// * 500: [StatusCode::INTERNAL_SERVER_ERROR] when a database error occurs.
#[axum::debug_handler]
pub async fn get_drawers(
    headers: HeaderMap,
    State(state): State<AppState>,
    params: Query<DrawerQueryOptions>,
) -> Result<Json<Vec<Drawer>>, (StatusCode, String)> {
    // Not going into detail on the errors since we already parsed the header in the middleware.
    let uuid = extract_headers_uuid(headers, &state).map_err(internal_error)?;

    // TODO: Implement user_id from auth-headers into the database request.
    use crate::schema::drawers::dsl::*;
    let conn = &mut establish_connection(state.db_url);
    // Set up boxed query to add pieces depending on query parameters.
    let mut query = drawers.into_boxed();
    // Pass user UUID as first parameter. Always required.
    query = query.filter(user_id.eq(uuid));

    let DrawerQueryOptions {
        d_id,
        drawer_name: d_name,
        freezer_id: f_id,
    } = params.deref();

    match (d_id, d_name, f_id) {
        (Some(_id), Some(_name), None) => {
            return Err((
                StatusCode::BAD_REQUEST,
                String::from("When a drawer_id is given, no other parameters can be given"),
            ));
        }
        (Some(_id), None, Some(_freezer_id)) => {
            return Err((
                StatusCode::BAD_REQUEST,
                String::from("When a drawer_id is given, no other parameters can be given"),
            ));
        }
        (Some(_id), Some(_name), Some(_freezer_id)) => {
            return Err((
                StatusCode::BAD_REQUEST,
                String::from("When a drawer_id is given, no other parameters can be given"),
            ));
        }
        (Some(d_id), None, None) => {
            query = query.filter(id.eq(d_id));
        }
        (None, Some(d_name), Some(f_id)) => {
            query = query.filter(name.eq(d_name)).filter(freezer_id.eq(f_id));
        }
        (None, Some(d_name), None) => {
            query = query.filter(name.eq(d_name));
        }
        (None, None, Some(f_id)) => {
            query = query.filter(freezer_id.eq(f_id));
        }
        _ => {
            let res = drawers.load::<Drawer>(conn).map_err(internal_error)?;
            return Ok(Json(res));
        }
    }

    let res = query.load::<Drawer>(conn).map_err(internal_error)?;

    Ok(Json(res))
}

/// Create a new product in the database: `POST /api/drawers`.
///
/// # Required body
///
/// [NewDrawer] model in `application/json`.
/// The drawer name must be unique within the same freezer.
///
/// # Returns
///
/// The new [Drawer].
///
/// # Errors
///
/// * `Duplicate` => "This drawer name already exists within this freezer".
pub async fn create_drawer(
    headers: HeaderMap,
    State(state): State<AppState>,
    new_drawer: Json<NewDrawer>,
) -> Result<Json<Drawer>, (StatusCode, String)> {
    let uuid = extract_headers_uuid(headers, &state).map_err(internal_error)?;

    use crate::schema::drawers::dsl::*;
    let conn = &mut establish_connection(state.db_url);
    let new_drawer = new_drawer.deref().to_owned();
    // // TODO: Check. May want to insert uuid in the frontend already.
    // new_drawer.user_id = uuid

    if new_drawer.user_id != uuid {
        return Err((
            StatusCode::FORBIDDEN,
            String::from("Authenticated User ID does not match user_id of drawer."),
        ));
    }

    let name_query = drawers
        .filter(user_id.eq(uuid))
        .filter(name.eq(&new_drawer.name))
        .filter(freezer_id.eq(&new_drawer.freezer_id))
        .get_results::<Drawer>(conn)
        .map_err(internal_error)?;

    if !name_query.is_empty() {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            String::from("This drawer name already exists within this freezer"),
        ));
    }

    let create_result = diesel::insert_into(drawers)
        .values(new_drawer)
        .returning(Drawer::as_returning())
        .get_result(conn)
        .map_err(internal_error)?;

    Ok(Json(create_result))
}

/// Updates a drawer in the database: `PATCH /api/drawers`. The frontend should never change
/// the drawer_id, only the [Drawer] name and [Drawer] freezer_id.
///
/// # Required body
///
/// [Drawer] model in `application/json`.
/// The drawer name must be unique within the same freezer.
///
/// # Returns
///
/// The updated [Drawer].
///
/// # Errors
///
/// * `Duplicate` => "This drawer name already exists within this freezer".
/// * `NotFound` => "Drawer not found". Returned when a wrong product_id was entered.
///
#[axum::debug_handler]
pub async fn update_drawer(
    headers: HeaderMap,
    State(state): State<AppState>,
    updated_drawer: Json<Drawer>,
) -> Result<Json<Drawer>, (StatusCode, String)> {
    let uuid = extract_headers_uuid(headers, &state).map_err(internal_error)?;

    use crate::schema::drawers::dsl::*;
    let conn = &mut establish_connection(state.db_url);
    let updated_drawer = updated_drawer.deref().to_owned();

    let name_query = drawers
        .filter(user_id.eq(uuid))
        .filter(name.eq(&updated_drawer.name))
        .filter(freezer_id.eq(&updated_drawer.freezer_id))
        .filter(id.ne(&updated_drawer.id))
        .get_results::<Drawer>(conn)
        .map_err(internal_error)?;

    if !name_query.is_empty() {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            String::from("This drawer name already exists within this freezer"),
        ));
    }

    let update_result = diesel::update(drawers)
        .filter(user_id.eq(uuid))
        .filter(id.eq(updated_drawer.id))
        .set(updated_drawer)
        .get_result(conn)
        .map_err(internal_error)?;

    Ok(Json(update_result))
}

/// Deletes a drawer in the database based on its `product_id`: `DELETE /api/drawers/id=<i32>`.
///
/// # Requires
///
/// A valid drawer ID to be given. It's recommended to implement delete protection in the frontend
/// as it may remove a lot of data linked to a drawer.
///
/// # Returns
///
/// The id of the deleted [Drawer].
///
/// # Errors
///
/// * `NotFound` => "Drawer not found".
#[axum::debug_handler]
pub async fn delete_drawer(
    headers: HeaderMap,
    State(state): State<AppState>,
    Path(delete_id): Path<i32>,
) -> Result<Json<i32>, (StatusCode, String)> {
    let uuid = extract_headers_uuid(headers, &state).map_err(internal_error)?;

    use crate::schema::drawers::dsl::*;
    let conn = &mut establish_connection(state.db_url);

    let id_query = drawers
        .filter(id.eq(&delete_id))
        .get_results::<Drawer>(conn)
        .map_err(internal_error)?;
    if id_query.is_empty() {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            String::from("Drawer not found"),
        ));
    }

    diesel::delete(drawers)
        .filter(user_id.eq(uuid))
        .filter(id.eq(delete_id))
        .execute(conn)
        .map_err(internal_error)?;

    Ok(Json(delete_id))
}
