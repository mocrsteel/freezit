//! [diesel.rs](http://diesel.rs) models.

use chrono::{NaiveDate, DateTime, Local};
use diesel::prelude::*;
use serde::{Serialize, Deserialize};
use typeshare::typeshare;

use crate::schema::{products, freezers, drawers, storage};

// Query | Select

/// **For testing purposes.** Type representing a [Product] database entry as a tuple.
pub type ProductTuple = (i32, &'static str, i32);

/// Product database model, matching [crate::schema::products].
///
/// This represents all the products that could be have been or are stored in one of the freezers.
/// The expiration time is used to calculate the expiration date of the different storage items in
/// the freezers and can be used to help the user which storage items should be consumed first.
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, Identifiable, Queryable, Selectable, AsChangeset, PartialEq, Eq)]
#[diesel(primary_key(product_id))]
#[diesel(table_name = products)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[serde(rename_all = "camelCase")]
pub struct Product {
    /// Product id.
    pub product_id: i32,
    /// Product name, must be unique and not longer than 50 characters.
    pub name: String,
    /// Time until product expires, defined in whole months. Defaults to 6 months if not given.
    pub expiration_months: i32,
}
impl Product {
    /// **For testing purposes.** Creates a product from a single tuple (statically
    /// defined in `tests/common/db_data.rs`).
    pub fn from_tuple(product: (i32, &str, i32)) -> Product {
        let (product_id, name, expiration_months) = product;

        Product {
            product_id,
            name: name.into(),
            expiration_months,
        }
    }
    /// **For testing purposes.** Creates a vector of products from a vector of tuples (statically
    /// defined in `tests/common/db_data.rs`).
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

/// **For testing purposes.** Type representing a [Freezer] database entry as a tuple.
pub type FreezerTuple = (i32, &'static str);

/// Freezer database model, matching [crate::schema::freezers].
///
/// This model represents the different freezers that might be in use at the user.
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, Identifiable, Queryable, Selectable, AsChangeset, Eq, PartialEq)]
#[diesel(primary_key(freezer_id))]
#[diesel(table_name = freezers)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[serde(rename_all = "camelCase")]
pub struct Freezer {
    /// Freezer id.
    pub freezer_id: i32,
    /// Freezer name, must be unique and not longer than 50 characters.
    pub name: String,
}
impl Freezer {
    /// **For testing purposes.** Creates a freezer from a single tuple (statically
    /// defined in `tests/common/db_data.rs`).
    pub fn from_tuple(freezer: (i32, &str)) -> Freezer {
        let (freezer_id, name) = freezer;

        Freezer {
            freezer_id,
            name: name.into()
        }
    }
    /// **For testing purposes.** Creates a vector of freezers from a vector of tuples (statically
    /// defined in `tests/common/db_data.rs`).
    pub fn from_vec(freezers: Vec<(i32, &str)>) -> Vec<Freezer> {
        freezers.into_iter().map(|(freezer_id, name)| {
            Freezer {
                freezer_id,
                name: name.into()
            }
        }).collect()
    }
}

/// **For testing purposes.** Type representing a [Drawer] database entry as a tuple.
pub type DrawerTuple = (i32, &'static str, i32);

/// Drawer database model, matching [crate::schema::drawers].
///
/// This represents the different drawers that could be present in the different freezers in use.
///
/// # Constraints
///
/// The combination of [Self::name] and [Self::freezer_id] must be unique.
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, Identifiable, Selectable, Queryable, Associations, AsChangeset, Eq, PartialEq)]
#[diesel(primary_key(drawer_id))]
#[diesel(belongs_to(Freezer, foreign_key = freezer_id))]
#[diesel(table_name = drawers)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[serde(rename_all = "camelCase")]
pub struct Drawer {
    /// Drawer id.
    pub drawer_id: i32,
    /// Drawer name, must be unique within the same [Self::freezer_id]
    pub name: String,
    /// Freezer id.
    pub freezer_id: i32,
}

impl Drawer {
    /// **For testing purposes.** Creates a drawers from a single tuple (statically
    /// defined in `tests/common/db_data.rs`).
    pub fn from_tuple(drawer: (i32, &str, i32)) -> Drawer {
        let (drawer_id, name, freezer_id) = drawer;

        Drawer {
           drawer_id,
            name: name.into(),
            freezer_id
        }
    }
    /// **For testing purposes.** Creates a vector of drawers from a vector of tuples (statically
    /// defined in `tests/common/db_data.rs`).
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


/// **For testing purposes.** Type representing a [Storage] database entry as a tuple.
pub type StorageTuple<'a> = (i32, i32, f32, &'a str, &'a str, bool, i32);

/// Storage database model, matching [crate::schema::storage].
///
/// This model represents all the different items that could be stored in any of the freezer drawers.
/// The date in will be either automatically set to the current date when not filled in, while the
/// date out will only be set once the product is withdrawn from the freezer.
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, Identifiable, Queryable, Selectable, Associations, AsChangeset)]
#[diesel(primary_key(storage_id))]
#[diesel(table_name = storage)]
#[diesel(belongs_to(Product, foreign_key = product_id))]
#[diesel(belongs_to(Drawer, foreign_key = drawer_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(treat_none_as_null = true)]
#[serde(rename_all = "camelCase")]
pub struct Storage {
    /// Storage id, serial number
    pub storage_id: i32,
    /// ID of product selected
    pub product_id: i32,
    /// Location of the product in the storage.
    pub drawer_id: i32,
    /// Weight of the product being stored, expressed in grams.
    pub weight_grams: f32,
    /// Date of storage, defaults to the current date. Input is stored based on the Utc DateTime,
    /// which is converted from `DateTime<Local>`
    pub date_in: NaiveDate,
    /// Date taken out of storage.
    pub date_out: Option<NaiveDate>,
    /// Whether or not the product is still in storage.
    pub available: bool,
}
impl Storage {
    /// **For testing purposes.** Creates a storage item from a single tuple (statically
    /// defined in `tests/common/db_data.rs`).
    pub fn from_tuple(storage: StorageTuple) -> Storage {
        let (storage_id, product_id, weight_grams, date_in, date_out, available, drawer_id) = storage;
        let date_in = DateTime::parse_from_str(format!("{} 12:00:00 +0200", date_in).as_str(), "%Y-%m-%d %H:%M:%S %z").unwrap().naive_utc().date();
        let date_out = match date_out {
            "" => None,
            _ => Some(DateTime::parse_from_str(format!("{} 12:00:00 +0200", date_in).as_str(), "%Y-%m-%d %H:%M:%S %z").unwrap().naive_utc().date()),
        };
        
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
    /// **For testing purposes.** Creates a vector of storage items from a vector of tuples (statically
    /// defined in `tests/common/db_data.rs`).
    pub fn from_vec(storages: Vec<StorageTuple>) -> Vec<Storage> {
        storages.into_iter().map(|(storage_id, product_id, weight_grams, date_in, date_out, available, drawer_id)| {
            let date_in = DateTime::parse_from_str(format!("{} 12:00:00 +0200", date_in).as_str(), "%Y-%m-%d %H:%M:%S %z").unwrap().naive_utc().date();
            let date_out = match date_out {
                "" => None,
                _ => Some(DateTime::parse_from_str(format!("{} 12:00:00 +0200", date_in).as_str(), "%Y-%m-%d %H:%M:%S %z").unwrap().naive_utc().date()),
            };
            
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
impl PartialEq for Storage {
    fn eq(&self, other: &Self) -> bool {
        self.weight_grams - other.weight_grams < 1e-6 &&
            self.storage_id == other.storage_id &&
            self.product_id == other.product_id &&
            self.drawer_id == other.drawer_id &&
            self.date_in == other.date_in &&
            self.date_out == other.date_out &&
            self.available == other.available

    }
}

// Insert

/// Insertable product containing the required fields.
#[typeshare]
#[derive(Debug, Clone, Deserialize, Serialize, Insertable)]
#[diesel(table_name = products)]
#[serde(rename_all = "camelCase")]
pub struct NewProduct {
    /// **Required, Unique**: The name of the product.
    pub name: String,
    /// **Optional**: The time until expiration in months. Defaults to 6 months.
    pub expiration_months: Option<i32>,
}

/// Insertable storage item containing the required fields.
#[typeshare]
#[derive(Debug, Clone, Deserialize, Serialize, Insertable)]
#[diesel(table_name = storage)]
#[serde(rename_all = "camelCase")]
pub struct NewStorageItem {
    /// **Required**: Id linked to product.
    pub product_id: i32,
    /// **Required**: ID of the drawer in which the product will be stored.
    pub drawer_id: i32,
    /// **Required**: The storage item weight, expressed in grams.
    pub weight_grams: f32,
    /// **Required**: Date in
    pub date_in: NaiveDate,
    /// **Required**: Availability, should be True on entry.
    pub available: bool,
}
impl NewStorageItem {
    /// Create new storage item. `date_in` is accepted as [Local] [DateTime].
    pub fn from(product_id: i32, drawer_id: i32, weight_grams: f32, date_in: DateTime<Local>) -> Self {
        let date_in = date_in.naive_utc().date();
        NewStorageItem {
            product_id,
            drawer_id,
            weight_grams,
            date_in,
            available: true
        }
    }
}

/// Allows storage availability update. Required to be able to set date_out to `NULL`.
#[derive(Debug, Clone, Deserialize, Serialize, AsChangeset)]
#[diesel(table_name = storage)]
#[serde(rename_all = "camelCase")]
pub struct UpdateStorageAvailability {
    /// [Storage] : field `available`
    pub available: bool,
    /// [Storage] : field `date_out`
    pub date_out: Option<NaiveDate>,
}

/// Insertable freezer containing the required fields.
#[typeshare]
#[derive(Debug, Clone, Deserialize, Serialize, Insertable)]
#[diesel(table_name = freezers)]
#[serde(rename_all = "camelCase")]
pub struct NewFreezer {
    /// **Required, Unique**: Freezer name.
    pub name: String,
}

/// Insertable freezer drawer containing the required fields.
#[typeshare]
#[derive(Debug, Clone, Deserialize, Serialize, Insertable)]
#[diesel(table_name = drawers)]
#[serde(rename_all = "camelCase")]
pub struct NewDrawer {
    /// **Required, Unique per `freezer_id`**: Drawer name.
    pub name: String,
    /// **Required**: Freezer id to which the drawer should be assigned to.
    pub freezer_id: i32,
}