//! Endpoint `/api/drawers`, implements `GET`, `POST`, `PATCH`, `DELETE`.

use axum::{extract::{Path, Query, State, Json}, http::StatusCode};
use diesel::{QueryDsl, RunQueryDsl};
use diesel::prelude::*;
use serde::Deserialize;
use std::ops::Deref;

use crate::AppState;
use crate::core::{
    connection::establish_connection,
    error::internal_error,
    query::empty_string_as_none,
};

use crate::models::{Drawer, NewDrawer};

/// Allowed query parameters to `GET` drawers. Any query parameters not in this struct will default to query all drawers.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DrawerQueryOptions {
    /// Id of the drawer.
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub drawer_id: Option<i32>,
    /// Id of the freezer containing one or more drawers.
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
pub async fn get_drawers(State(state): State<AppState>, params: Query<DrawerQueryOptions>) -> Result<Json<Vec<Drawer>>, (StatusCode, String)>
{
    use crate::schema::drawers::dsl::*;
    let conn = &mut establish_connection(state.db_url);
    // Set up boxed query to add pieces depending on query parameters.
    let mut query = drawers.into_boxed();
    let DrawerQueryOptions { drawer_id: d_id, drawer_name: d_name, freezer_id: f_id } = params.deref();

    match (d_id, d_name, f_id) {
        (Some(_id), Some(_name), None) => {
            return Err((StatusCode::BAD_REQUEST, String::from("When a drawer_id is given, no other parameters can be given")));
        }
        (Some(_id), None, Some(_freezer_id)) => {
            return Err((StatusCode::BAD_REQUEST, String::from("When a drawer_id is given, no other parameters can be given")));
        }
        (Some(_id), Some(_name), Some(_freezer_id)) => {
            return Err((StatusCode::BAD_REQUEST, String::from("When a drawer_id is given, no other parameters can be given")));
        }
        (Some(id), None, None) => {
            query = query.filter(drawer_id.eq(id));
        }
        (None, Some(d_name), Some(f_id)) => {
            query = query.filter(name.eq(d_name))
                .filter(freezer_id.eq(f_id));
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

    let res = query
        .load::<Drawer>(conn)
        .map_err(internal_error)?;

    Ok(Json(res))
}

/// Create a new product in the database: `POST /api/drawers`.
///
/// # Required body
///
/// [NewDrawer] model in `application/json'.
/// The drawer name must be unique within the same freezer.
///
/// # Returns
///
/// The new [Drawer].
///
/// # Errors
///
/// * `Duplicate` => "This drawer name already exists within this freezer".
pub async fn create_drawer(State(state): State<AppState>, new_drawer: Json<NewDrawer>) -> Result<Json<Drawer>, (StatusCode, String)> {
    use crate::schema::drawers::dsl::*;
    let conn = &mut establish_connection(state.db_url);
    let new_drawer = new_drawer.deref().to_owned();

    let name_query = drawers
        .filter(name.eq(&new_drawer.name))
        .filter(freezer_id.eq(&new_drawer.freezer_id))
        .get_results::<Drawer>(conn)
        .map_err(internal_error)?;

    if !name_query.is_empty() {
        return Err((StatusCode::INTERNAL_SERVER_ERROR, String::from("This drawer name already exists within this freezer")));
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
/// [Drawer] model in `application/json'.
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
pub async fn update_drawer(State(state): State<AppState>, updated_drawer: Json<Drawer>) -> Result<Json<Drawer>, (StatusCode, String)> {
    use crate::schema::drawers::dsl::*;
    let conn = &mut establish_connection(state.db_url);
    let updated_drawer = updated_drawer.deref().to_owned();

    let name_query = drawers
        .filter(name.eq(&updated_drawer.name))
        .filter(freezer_id.eq(&updated_drawer.freezer_id))
        .filter(drawer_id.ne(&updated_drawer.drawer_id))
        .get_results::<Drawer>(conn)
        .map_err(internal_error)?;

    if !name_query.is_empty() {
        return Err((StatusCode::INTERNAL_SERVER_ERROR, String::from("This drawer name already exists within this freezer")));
    }

    let update_result = diesel::update(drawers)
        .filter(drawer_id.eq(updated_drawer.drawer_id))
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
pub async fn delete_drawer(State(state): State<AppState>, Path(id): Path<i32>) -> Result<Json<i32>, (StatusCode, String)> {
    use crate::schema::drawers::dsl::*;
    let conn = &mut establish_connection(state.db_url);

    let id_query = drawers
        .filter(drawer_id.eq(&id))
        .get_results::<Drawer>(conn)
        .map_err(internal_error)?;
    if id_query.is_empty() {
        return Err((StatusCode::INTERNAL_SERVER_ERROR, String::from("Drawer not found")));
    }

    diesel::delete(drawers)
        .filter(drawer_id.eq(id))
        .execute(conn)
        .map_err(internal_error)?;

    Ok(Json(id))
}
