use chrono::NaiveDate;
use diesel::prelude::*;
use typeshare::typeshare;

use crate::schema::{products, freezers, drawers, storage};

// Query | Select

pub type ProductTuple = (i32, &'static str, i32);

#[typeshare]
#[derive(serde::Serialize, Queryable, Selectable, AsChangeset)]
#[diesel(table_name = products)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[serde(rename_all = "camelCase")]
pub struct Product {
    pub product_id: i32,
    pub name: String,
    pub expiration_months: i32,
}
impl Product {
    pub fn from_tuple(product: (i32, &str, i32)) -> Product {
        let (product_id, name, expiration_months) = product;

        Product {
            product_id,
            name: name.into(),
            expiration_months,
        }
    }
    pub fn from_vec(products: Vec<(i32, &str, i32)>) -> Vec<Product> {
        products.into_iter().map(|(product_id, name, expiration_months)| {
            Product {
                product_id,
                name: name.into(),
                expiration_months
            }
        }).collect()
    }
}

pub type FreezerTuple = (i32, &'static str);

#[typeshare]
#[derive(serde::Serialize, Queryable, Selectable, AsChangeset)]
#[diesel(table_name = freezers)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[serde(rename_all = "camelCase")]
pub struct Freezer {
    pub freezer_id: i32,
    pub name: String,
}
impl Freezer {
    pub fn from_tuple(freezer: (i32, &str)) -> Freezer {
        let (freezer_id, name) = freezer;

        Freezer {
            freezer_id,
            name: name.into()
        }
    }
    pub fn from_vec(freezers: Vec<(i32, &str)>) -> Vec<Freezer> {
        freezers.into_iter().map(|(freezer_id, name)| {
            Freezer {
                freezer_id,
                name: name.into()
            }
        }).collect()
    }
}

pub type DrawerTuple = (i32, &'static str, i32);

#[typeshare]
#[derive(serde::Serialize, Queryable, Selectable, AsChangeset)]
#[diesel(table_name = drawers)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[serde(rename_all = "camelCase")]
pub struct Drawer {
    pub drawer_id: i32,
    pub name: String,
    pub freezer_id: i32,
}
impl Drawer {
    pub fn from_tuple(drawer: (i32, &str, i32)) -> Drawer {
        let (drawer_id, name, freezer_id) = drawer;

        Drawer {
           drawer_id,
            name: name.into(),
            freezer_id
        }
    }
    pub fn from_vec(drawers: Vec<(i32, &str, i32)>) -> Vec<Drawer> {
        drawers.into_iter().map(|(drawer_id, name, freezer_id)| {
            Drawer {
                drawer_id,
                name: name.into(),
                freezer_id
            }
        }).collect()
    }
}

pub type StorageTuple = (i32, i32, f32, NaiveDate, Option<NaiveDate>, bool, i32);

#[typeshare]
#[derive(serde::Serialize, Queryable, Selectable, Associations, AsChangeset)]
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
    pub drawer_id: i32,
}
impl Storage {
    pub fn from_tuple(storage: StorageTuple) -> Storage {
        let (storage_id, product_id, weight_grams, date_in, date_out, available, drawer_id) = storage;

        Storage {
            storage_id,
            product_id,
            weight_grams,
            date_in,
            date_out,
            available,
            drawer_id,
        }
    }
    pub fn from_vec(storages: Vec<StorageTuple>) -> Vec<Storage> {
        storages.into_iter().map(|(storage_id, product_id, weight_grams, date_in, date_out, available, drawer_id)| {
            Storage {
                storage_id,
                product_id,
                weight_grams,
                date_in,
                date_out,
                available,
                drawer_id,
            }
        }).collect()
    }
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
    pub drawer_id: i32,
}

#[typeshare]
#[derive(serde::Deserialize, Insertable)]
#[diesel(table_name = freezers)]
#[serde(rename_all = "camelCase")]
pub struct NewFreezer<'a> {
    pub name: &'a str,
}

#[typeshare]
#[derive(serde::Deserialize, Insertable)]
#[diesel(table_name = drawers)]
#[serde(rename_all = "camelCase")]
pub struct NewDrawer<'a> {
    pub name: &'a str,
    pub freezer_id: i32,
}