use axum::{
    extract::Path,
    response::{Json, },
    http::StatusCode,
};
use diesel::QueryDsl;
use diesel::prelude::*;

use crate::error::internal_error;
use crate::models::Product;
use crate::connection::establish_connection;
use crate::schema::products::dsl::products;

pub async fn get_product_by_id(Path(id): Path<i32>) -> Result<Json<Product>, (StatusCode, String)> {
    use crate::schema::products::dsl::*;
    let conn = &mut establish_connection();

    let res = products
        .filter(product_id.eq(id))
        .select(Product::as_select())
        .first(conn)
        .map_err(internal_error)?;

    Ok(Json(res))
}

pub async fn get_product_by_name(Path(query_name): Path<String>) -> Result<Json<Product>, (StatusCode, String)> {
    use crate::schema::products::dsl::*;
    let conn = &mut establish_connection();

    let res = products
        .filter(name.eq(query_name))
        .select(Product::as_select())
        .first(conn)
        .map_err(internal_error)?;

    Ok(Json(res))
}
