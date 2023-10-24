use chrono::NaiveDate;
use diesel::prelude::*;

use crate::schema::{products, storage};

// Query | Select
#[derive(Queryable, Selectable)]
#[diesel(table_name = products)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Product {
    pub product_id: i32,
    pub name: String,
    pub expiration_months: i32,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = storage)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Storage {
    pub storage_id: i32,
    pub product_id: i32,
    pub weight_grams: f32,
    pub date_in: NaiveDate,
    pub date_out: Option<NaiveDate>,
    pub available: bool,
}

// Insert
#[derive(Insertable)]
#[diesel(table_name = products)]
pub struct NewProduct<'a> {
    pub name: &'a str,
    pub expiration_months: Option<i32>,
}

#[derive(Insertable)]
#[diesel(table_name = storage)]
pub struct NewStorageItem {
    pub product_id: i32,
    pub weight_grams: f32,
    pub date_in: NaiveDate,
    pub available: bool,
}
