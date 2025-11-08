//! [diesel.rs](http://diesel.rs) models.

use crate::schema::{drawers, email_whitelist, freezers, products, storage, users};

use chrono::NaiveDate;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;
use uuid::Uuid;

// Query | Select

/// User database model, matching [create::schema::users].
///
/// This represents the users stored in the database.
#[typeshare]
#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
    Identifiable,
    Queryable,
    Selectable,
    AsChangeset,
    PartialEq,
    Eq,
)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[serde(rename_all = "camelCase")]
pub struct User {
    /// User UUID (not a regular `i64` id!). Example: `4d04bdf0-6bd0-4fec-b42b-070d10def093`.
    pub id: Uuid,
    /// User's full name. Should be space separated: `<first> <last>`
    pub name: String,
    /// User's email address.
    pub email: String,
}
impl User {
    /// **Testing purposes only** Generate an User instance from a tuple.
    #[cfg(test)]
    pub fn from_tuple(user: (Uuid, &str, &str)) -> User {
        let (id, name, email) = user;
        User {
            id,
            name: name.into(),
            email: email.into(),
        }
    }

    /// **Testing purposes only** Generate a `Vec` of `User` for testing.
    #[cfg(any(test, feature = "seed"))]
    pub fn from_vec(user: Vec<(Uuid, &str, &str)>) -> Vec<User> {
        user.into_iter()
            .map(|(id, name, email)| User {
                id,
                name: name.into(),
                email: email.into(),
            })
            .collect()
    }
}

/// Table containing whitelisted email addresses.
///
/// The design intent of the app is to restrict the app to specific users (close circle of developper).
/// For that purpose, a email whitelist will be generated. If a user signs up, his email will
/// be crosschecked against the whitelist and a UUID will be granted for tracking.
#[typeshare]
#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
    Identifiable,
    Queryable,
    Selectable,
    AsChangeset,
    PartialEq,
    Eq,
)]
#[diesel(table_name = email_whitelist)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[serde(rename_all = "camelCase")]
pub struct EmailWhitelist {
    /// Entry id
    pub id: i32,
    /// Whitelisted email address.
    pub email: String,
}

/// **For testing purposes.** Type representing a [Product] database entry as a tuple.
#[cfg(test)]
pub type ProductTuple = (i32, &'static str, i32);

/// Product database model, matching [crate::schema::products].
///
/// This represents all the products that could be have been or are stored in one of the freezers.
/// The expiration time is used to calculate the expiration date of the different storage items in
/// the freezers and can be used to help the user which storage items should be consumed first.
#[typeshare]
#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
    Identifiable,
    Queryable,
    Selectable,
    AsChangeset,
    PartialEq,
    Eq,
)]
#[diesel(table_name = products)]
#[diesel(belongs_to(User, foreign_key = user_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[serde(rename_all = "camelCase")]
pub struct Product {
    /// Product id.
    pub id: i32,
    /// Product name, must be unique and not longer than 50 characters.
    pub name: String,
    /// Time until product expires, defined in whole months. Defaults to 6 months if not given.
    pub expiration_months: i32,
    /// User to which the product is linked.
    pub user_id: Uuid,
}
impl Product {
    /// **For testing purposes.** Creates a product from a single tuple (statically
    /// defined in `tests/common/db_data.rs`).
    #[cfg(test)]
    pub fn from_tuple(product: (i32, &str, i32, Uuid)) -> Product {
        let (id, name, expiration_months, user_id) = product;

        Product {
            id,
            name: name.into(),
            expiration_months,
            user_id,
        }
    }
    /// **For testing purposes.** Creates a vector of products from a vector of tuples (statically
    /// defined in `tests/common/db_data.rs`).
    #[cfg(any(test, feature = "seed"))]
    pub fn from_vec(products: Vec<(i32, &str, i32, &str)>) -> Vec<Product> {
        products
            .into_iter()
            .map(|(id, name, expiration_months, user_id)| {
                let user_id = Uuid::parse_str(user_id).unwrap();
                Product {
                    id,
                    name: name.into(),
                    expiration_months,
                    user_id,
                }
            })
            .collect()
    }
}

/// **For testing purposes.** Type representing a [Freezer] database entry as a tuple.
#[cfg(test)]
pub type FreezerTuple = (i32, &'static str);

/// Freezer database model, matching [crate::schema::freezers].
///
/// This model represents the different freezers that might be in use at the user.
#[typeshare]
#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
    Identifiable,
    Queryable,
    Selectable,
    AsChangeset,
    Eq,
    PartialEq,
)]
#[diesel(table_name = freezers)]
#[diesel(belongs_to(User, foreign_key = user_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[serde(rename_all = "camelCase")]
pub struct Freezer {
    /// Freezer id.
    pub id: i32,
    /// Freezer name, must be unique and not longer than 50 characters.
    pub name: String,
    /// User to which the freezer is linked.
    pub user_id: Uuid,
}
impl Freezer {
    /// **For testing purposes.** Creates a freezer from a single tuple (statically
    /// defined in `tests/common/db_data.rs`).
    #[cfg(test)]
    pub fn from_tuple(freezer: (i32, &str, Uuid)) -> Freezer {
        let (id, name, user_id) = freezer;

        Freezer {
            id,
            name: name.into(),
            user_id,
        }
    }
    /// **For testing purposes.** Creates a vector of freezers from a vector of tuples (statically
    /// defined in `tests/common/db_data.rs`).
    #[cfg(any(test, feature = "seed"))]
    pub fn from_vec(freezers: Vec<(i32, &str, &str)>) -> Vec<Freezer> {
        freezers
            .into_iter()
            .map(|(id, name, user_id)| {
                    let user_id = Uuid::parse_str(user_id).unwrap();
                    Freezer {
                        id,
                        name: name.into(),
                        user_id,
                    }
                }
            )
            .collect()
    }
}

/// **For testing purposes.** Type representing a [Drawer] database entry as a tuple.
#[cfg(test)]
pub type DrawerTuple = (i32, &'static str, i32);

/// Drawer database model, matching [crate::schema::drawers].
///
/// This represents the different drawers that could be present in the different freezers in use.
///
/// # Constraints
///
/// The combination of [Self::name] and [Self::freezer_id] must be unique.
#[typeshare]
#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
    Identifiable,
    Selectable,
    Queryable,
    Associations,
    AsChangeset,
    Eq,
    PartialEq,
)]
#[diesel(belongs_to(Freezer, foreign_key = freezer_id))]
#[diesel(belongs_to(User, foreign_key = user_id))]
#[diesel(table_name = drawers)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[serde(rename_all = "camelCase")]
pub struct Drawer {
    /// Drawer id.
    pub id: i32,
    /// Drawer name, must be unique within the same [Self::freezer_id]
    pub name: String,
    /// Freezer id.
    pub freezer_id: i32,
    /// User to which the drawer is linked.
    pub user_id: Uuid,
}

impl Drawer {
    /// **For testing purposes.** Creates a drawers from a single tuple (statically
    /// defined in `tests/common/db_data.rs`).
    #[cfg(test)]
    pub fn from_tuple(drawer: (i32, &str, i32, Uuid)) -> Drawer {
        let (id, name, freezer_id, user_id) = drawer;

        Drawer {
            id,
            name: name.into(),
            freezer_id,
            user_id,
        }
    }
    /// **For testing purposes.** Creates a vector of drawers from a vector of tuples (statically
    /// defined in `tests/common/db_data.rs`).
    #[cfg(any(test, feature = "seed"))]
    pub fn from_vec(drawers: Vec<(i32, &str, i32, &str)>) -> Vec<Drawer> {
        drawers
            .into_iter()
            .map(|(id, name, freezer_id, user_id)| {
                let user_id = Uuid::parse_str(user_id).unwrap();
                Drawer {
                    id,
                    name: name.into(),
                    freezer_id,
                    user_id,
                }
            })
            .collect()
    }
}

/// **For testing purposes.** Type representing a [Storage] database entry as a tuple.
pub type StorageTuple<'a> = (i32, i32, f32, &'a str, &'a str, i32, &'a str);

/// Storage database model, matching [crate::schema::storage].
///
/// This model represents all the different items that could be stored in any of the freezer drawers.
/// The date in will be either automatically set to the current date when not filled in, while the
/// date out will only be set once the product is withdrawn from the freezer.
#[typeshare]
#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
    Identifiable,
    Queryable,
    Selectable,
    Associations,
    AsChangeset,
)]
#[diesel(table_name = storage)]
#[diesel(belongs_to(Product, foreign_key = product_id))]
#[diesel(belongs_to(Drawer, foreign_key = drawer_id))]
#[diesel(belongs_to(User, foreign_key = user_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(treat_none_as_null = true)]
#[serde(rename_all = "camelCase")]
pub struct Storage {
    /// Storage id, serial number
    pub id: i32,
    /// ID of product selected
    pub product_id: i32,
    /// Location of the product in the storage.
    pub drawer_id: i32,
    /// Weight of the product being stored, expressed in grams.
    pub weight_grams: f32,
    /// Date of storage, defaults to the current date. Derived from `DateTime<Local>` and parsed into Date string.
    pub date_in: NaiveDate,
    /// Date taken out of storage.
    pub date_out: Option<NaiveDate>,
    /// User to which the storage entry is linked.
    pub user_id: Uuid,
}
impl Storage {
    /// **For testing purposes.** Creates a storage item from a single tuple (statically
    /// defined in `tests/common/db_data.rs`).
    #[cfg(test)]
    pub fn from_tuple(storage: StorageTuple) -> Storage {
        let (id, product_id, weight_grams, date_in, date_out, drawer_id, user_id) = storage;
        let date_in = NaiveDate::parse_from_str(date_in, "%Y-%m-%d").unwrap();
        // let date_in = DateTime::parse_from_str(format!("{} 12:00:00 +0200", date_in).as_str(), "%Y-%m-%d %H:%M:%S %z").unwrap().naive_utc().date();
        let date_out = match date_out {
            "" => None,
            _ => Some(NaiveDate::parse_from_str(date_out, "%Y-%m-%d").unwrap()),
            // _ => Some(DateTime::parse_from_str(format!("{} 12:00:00 +0200", date_in).as_str(), "%Y-%m-%d %H:%M:%S %z").unwrap().naive_utc().date()),
        };
        let user_id = Uuid::parse_str(user_id).unwrap();
        Storage {
            id,
            product_id,
            weight_grams,
            date_in,
            date_out,
            drawer_id,
            user_id,
        }
    }
    /// **For testing purposes.** Creates a vector of storage items from a vector of tuples (statically
    /// defined in `tests/common/db_data.rs`).
    #[cfg(any(test, feature = "seed"))]
    pub fn from_vec(storages: Vec<StorageTuple>) -> Vec<Storage> {
        storages
            .into_iter()
            .map(
                |(id, product_id, weight_grams, date_in, date_out, drawer_id, user_id)| {
                    let date_in = NaiveDate::parse_from_str(date_in, "%Y-%m-%d").unwrap();
                    // let date_in = DateTime::parse_from_str(format!("{} 12:00:00 +0200", date_in).as_str(), "%Y-%m-%d %H:%M:%S %z").unwrap().naive_utc().date();
                    let date_out = match date_out {
                        "" => None,
                        _ => Some(NaiveDate::parse_from_str(date_out, "%Y-%m-%d").unwrap()),
                        // _ => Some(DateTime::parse_from_str(format!("{} 12:00:00 +0200", date_in).as_str(), "%Y-%m-%d %H:%M:%S %z").unwrap().naive_utc().date()),
                    };
                    let user_id = Uuid::parse_str(user_id).unwrap();
                    Storage {
                        id,
                        product_id,
                        weight_grams,
                        date_in,
                        date_out,
                        drawer_id,
                        user_id,
                    }
                },
            )
            .collect()
    }
}
impl PartialEq for Storage {
    fn eq(&self, other: &Self) -> bool {
        self.weight_grams - other.weight_grams < 1e-6
            && self.id == other.id
            && self.product_id == other.product_id
            && self.drawer_id == other.drawer_id
            && self.date_in == other.date_in
            && self.date_out == other.date_out
            && self.user_id == other.user_id
    }
}

// Insert

/// Insertable user.
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[diesel(table_name = users)]
#[serde(rename_all = "camelCase")]
pub struct NewUser {
    /// **Required**: Name of the new user.
    pub name: String,
    /// **Required, Unique**: Email of the user.
    pub email: String,
}

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
    /// **Required**: User to which this entry will belong.
    pub user_id: Uuid,
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
    /// **Required**: User to which the storage item belongs.
    pub user_id: Uuid,
}
impl NewStorageItem {
    /// Create new storage item. `date_in` is accepted as [Local] [DateTime].
    pub fn from(
        product_id: i32,
        drawer_id: i32,
        weight_grams: f32,
        date_in: NaiveDate,
        user_id: Uuid,
    ) -> Self {
        NewStorageItem {
            product_id,
            drawer_id,
            weight_grams,
            date_in,
            user_id,
        }
    }
}

/// Allows storage availability update. Required to be able to set date_out to `NULL`.
#[derive(Debug, Clone, Deserialize, Serialize, AsChangeset)]
#[diesel(table_name = storage)]
#[diesel(treat_none_as_null = true)]
#[serde(rename_all = "camelCase")]
pub struct UpdateStorageAvailability {
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
    /// **Required**: User to which the freezer belongs.
    pub user_id: Uuid,
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
    /// **Required**: User to which the drawer belongs.
    pub user_id: Uuid,
}

/// Insertable email to be whitelisted.
#[typeshare]
#[derive(Debug, Clone, Deserialize, Serialize, Insertable)]
#[diesel(table_name = email_whitelist)]
#[serde(rename_all = "camelCase")]
pub struct NewEmailWhitelist {
    /// User email address. The frontend has to ensure the email is in a valid format.
    pub email: String,
}

/// Struct for seeding the database
#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = users)]
pub struct SeedUser {
    /// uuid
    pub id: Uuid,
    /// email
    pub email: String,
    /// name
    pub name: String,
}
