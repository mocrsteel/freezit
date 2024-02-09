//! Endpoint `/api/products`, implements `GET`, `POST`, `PATCH`, `DELETE`.

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};
use diesel::prelude::*;
use diesel::QueryDsl;
use std::ops::Deref;

use crate::core::{
    connection::establish_connection,
    error::internal_error
};
use crate::models::{NewProduct, Product};
use crate::AppState;

/// Get a product entry by its ID, given as a path parameter: `GET /api/products/id=<i32>`.
///
/// # Returns
///
/// A single product entry (no duplicate names allowed)
///
/// # Errors
///
/// * `NotFound` => "Product not found".
pub async fn get_product_by_id(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<Product>, (StatusCode, String)> {
    use crate::schema::products::dsl::*;
    let conn = &mut establish_connection(state.db_url);

    let res = products
        .filter(product_id.eq(id))
        .select(Product::as_select())
        .first(conn)
        .map_err(internal_error)?;

    Ok(Json(res))
}

/// Get a product entry by its name, given as a path parameter: `GET /api/products/name=<String>`.
///
/// # Returns
///
/// A single product entry (no duplicate names allowed)
///
/// # Errors
///
/// * `NotFound` => "Product not found".
pub async fn get_product_by_name(
    State(state): State<AppState>,
    Path(query_name): Path<String>,
) -> Result<Json<Product>, (StatusCode, String)> {
    use crate::schema::products::dsl::*;
    let conn = &mut establish_connection(state.db_url);

    let res = products
        .filter(name.eq(query_name))
        .select(Product::as_select())
        .first(conn)
        .map_err(internal_error)?;

    Ok(Json(res))
}

/// Get products based on their expiration time in months, given as a path parameter:
/// `GET /api/products/expiration_months=<i32>`.
///
/// # Returns
///
/// A vector of products.
///
/// # Errors
///
/// * `ExpirationNotFound` => "No products defined with this expiration time".
pub async fn get_products_by_expiration(
    State(state): State<AppState>,
    Path(query_expiration): Path<i32>,
) -> Result<Json<Vec<Product>>, (StatusCode, String)> {
    use crate::schema::products::dsl::*;
    let conn = &mut establish_connection(state.db_url);

    let res: Vec<Product> = products
        .filter(expiration_months.eq(query_expiration))
        .get_results(conn)
        .map_err(internal_error)?;

    Ok(Json(res))
}

/// Get all products stored in the database: `GET /api/products`.
///
/// # Returns
///
/// A vector of products.
///
/// # Errors
///
/// * `NotFound` => "Product not found". Only returned on an empty database.
pub async fn get_all_products(
    State(state): State<AppState>,
) -> Result<Json<Vec<Product>>, (StatusCode, String)> {
    use crate::schema::products::dsl::*;
    let conn = &mut establish_connection(state.db_url);

    let res = products.load::<Product>(conn).map_err(internal_error)?;

    Ok(Json(res))
}

/// Create a new product in the database: `POST /api/products`.
///
/// # Required body
///
/// [NewProduct] model in `application/json'.
/// The product name must be unique.
///
/// # Returns
///
/// The new [Product].
///
/// # Errors
///
/// * `Duplicate` => "This product name already exists".
pub async fn create_product(
    State(state): State<AppState>,
    new_product: Json<NewProduct>,
) -> Result<Json<Product>, (StatusCode, String)> {
    use crate::schema::products::dsl::*;
    let conn = &mut establish_connection(state.db_url);
    let new_product = new_product.deref().to_owned();

    let name_query = products
        .filter(name.eq(&new_product.name))
        .get_results::<Product>(conn)
        .map_err(internal_error)?;

    if !name_query.is_empty() {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            String::from("This product name already exists"),
        ));
    }

    let res = diesel::insert_into(products)
        .values(new_product)
        .returning(Product::as_returning())
        .get_result(conn)
        .map_err(internal_error)?;

    Ok(Json(res))
}

/// Updates a new product in the database: `PATCH /api/products`. The frontend should never change
/// the product id, only the [Product] name and [Product] expiration_months.
///
/// # Required body
///
/// [Product] model in `application/json'.
/// The product name must be unique.
///
/// # Returns
///
/// The updated [Product].
///
/// # Errors
///
/// * `Duplicate` => "This product name already exists".
/// * `NotFound` => "Product not found". Returned when a wrong product_id was entered.
///
pub async fn update_product(
    State(state): State<AppState>,
    update_product: Json<Product>,
) -> Result<Json<Product>, (StatusCode, String)> {
    use crate::schema::products::dsl::*;
    let conn = &mut establish_connection(state.db_url);
    let updated_product = update_product.deref().to_owned();

    let name_lookup = products
        .filter(product_id.ne(&update_product.product_id))
        .filter(name.eq(&update_product.name))
        .get_results::<Product>(conn)
        .map_err(internal_error)?;

    if !name_lookup.is_empty() {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            String::from("This product name already exists"),
        ));
    }

    let res = diesel::update(products)
        .filter(product_id.eq(&updated_product.product_id))
        .set(&updated_product)
        .returning(Product::as_returning())
        .get_result(conn)
        .map_err(internal_error)?;

    Ok(Json(res))
}

/// Deletes a product in the database based on its `product_id`: `DELETE /api/products/id=<i32>`.
///
/// # Requires
///
/// A valid product ID to be given. It's recommended to implement delete protection in the frontend
/// as it may remove a lot of data linked to a product.
///
/// # Returns
///
/// The deleted [Product] id.
///
/// # Errors
///
/// * `NotFound` => "Product not found". Returned when a wrong product_id was entered.
///
pub async fn delete_product(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<i32>, (StatusCode, String)> {
    use crate::schema::products::dsl::*;
    let conn = &mut establish_connection(state.db_url);

    let id_query = products
        .find(id)
        .get_results::<Product>(conn)
        .map_err(internal_error)?;
    if id_query.is_empty() {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            String::from("This product id does not exist")
        ));
    }

    diesel::delete(products)
        .filter(product_id.eq(id))
        .execute(conn)
        .map_err(internal_error)?;

    Ok(Json(id))
}

/// Counts the amount of occurrences of a product in the storage table:
/// `GET /api/products/storage?id=<i32>`.
///
/// # Returns
///
/// `i32`: amount of products with given id in the [crate::schema::storage] table.
///
/// # Errors
///
/// None, returns `0` if nothing is found.
#[allow(dead_code)]
async fn count_products_storage() {}
