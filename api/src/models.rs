use chrono::NaiveDate;
use diesel::prelude::*;
use typeshare::typeshare;

use crate::schema::{products, storage};

// Query | Select
#[typeshare]
#[derive(serde::Serialize, Queryable, Selectable)]
#[diesel(table_name = products)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[serde(rename_all = "camelCase")]
pub struct Product {
    pub product_id: i32,
    pub name: String,
    pub expiration_months: i32,
}

#[typeshare]
#[derive(serde::Serialize, Queryable, Selectable, Associations)]
#[diesel(table_name = storage)]
#[diesel(belongs_to(Product))]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[serde(rename_all = "camelCase")]
pub struct Storage {
    pub storage_id: i32,
    pub product_id: i32,
    pub weight_grams: f32,
    pub date_in: NaiveDate,
    pub date_out: Option<NaiveDate>,
    pub available: bool,
}

// Insert
#[typeshare]
#[derive(serde::Deserialize, Insertable)]
#[diesel(table_name = products)]
#[serde(rename_all = "camelCase")]
pub struct NewProduct<'a> {
    pub name: &'a str,
    pub expiration_months: Option<i32>,
}

#[typeshare]
#[derive(serde::Deserialize, Insertable)]
#[diesel(table_name = storage)]
#[serde(rename_all = "camelCase")]
pub struct NewStorageItem {
    pub product_id: i32,
    pub weight_grams: f32,
    pub date_in: NaiveDate,
    pub available: bool,
}
