use axum::{
    extract::{Path, State},
    response::{Json, },
    http::StatusCode,
};
use diesel::QueryDsl;
use diesel::prelude::*;

use crate::AppState;
use crate::error::internal_error;
use crate::models::Product;
use crate::connection::establish_connection;

pub async fn get_product_by_id(State(state): State<AppState>, Path(id): Path<i32>) -> Result<Json<Product>, (StatusCode, String)> {
    use crate::schema::products::dsl::*;
    let conn = &mut establish_connection(state.db_url);

    let res = products
        .filter(product_id.eq(id))
        .select(Product::as_select())
        .first(conn)
        .map_err(internal_error)?;

    Ok(Json(res))
}

pub async fn get_product_by_name(State(state): State<AppState>, Path(query_name): Path<String>) -> Result<Json<Product>, (StatusCode, String)> {
    use crate::schema::products::dsl::*;
    let conn = &mut establish_connection(state.db_url);

    let res = products
        .filter(name.eq(query_name))
        .select(Product::as_select())
        .first(conn)
        .map_err(internal_error)?;

    Ok(Json(res))
}

pub async fn get_products_by_expiration(State(state): State<AppState>, Path(query_expiration): Path<i32>) -> Result<Json<Vec<Product>>, (StatusCode, String)> {
    use crate::schema::products::dsl::*;
    let conn = &mut establish_connection(state.db_url);

    let res: Vec<Product> = products
        .filter(expiration_months.eq(query_expiration))
        .get_results(conn)
        .map_err(internal_error)?;

    Ok(Json(res))
}

async fn update_product() {

}

async fn delete_product() {

}
