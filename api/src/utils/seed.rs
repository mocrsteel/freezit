use crate::{
    core::connection::establish_connection, mock_data, models::*,
    schema::drawers::dsl as drawers_dsl, schema::email_whitelist::dsl as whitelist_dsl,
    schema::freezers::dsl as freezers_dsl, schema::products::dsl as products_dsl,
    schema::storage::dsl as storage_dsl, schema::users::dsl as users_dsl,
};
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;


/// Function to seed a development database for testing/playing around.
pub fn seed() {
    // const MIGRATIONS: EmbeddedMigrations = embed_migrations!();
    // fn run_migrations(
    //     connection: &mut impl MigrationHarness<Pg>,
    // ) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    //     // This will run the necessary migrations.
    //     //
    //     // See the documentation for `MigrationHarness` for
    //     // all available methods.
    //     connection.run_pending_migrations(MIGRATIONS)?;
    //
    //     Ok(())
    // }

    dotenv().ok();

    println!("Seeding database with dunmmy data...");
    let users = User::from_vec(mock_data::USERS.to_vec())
        .into_iter()
        .map(|User { id, name, email }| SeedUser { id, email, name })
        .collect::<Vec<SeedUser>>();

    let email_whitelist = users
        .clone()
        .into_iter()
        .map(|SeedUser { email, .. }| NewEmailWhitelist { email })
        .collect::<Vec<NewEmailWhitelist>>();

    let freezers = Freezer::from_vec(mock_data::FREEZERS.to_vec())
        .into_iter()
        .map(|Freezer { name, user_id, .. }| NewFreezer { name, user_id })
        .collect::<Vec<NewFreezer>>();

    let drawers = Drawer::from_vec(mock_data::DRAWERS.to_vec())
        .into_iter()
        .map(
            |Drawer {
                 name,
                 user_id,
                 freezer_id,
                 ..
             }| {
                NewDrawer {
                    name,
                    freezer_id,
                    user_id,
                }
            },
        )
        .collect::<Vec<NewDrawer>>();

    let products = Product::from_vec(mock_data::PRODUCTS.to_vec())
        .into_iter()
        .map(
            |Product {
                 name,
                 user_id,
                 expiration_months,
                 ..
             }| {
                NewProduct {
                    name,
                    expiration_months: Some(expiration_months),
                    user_id,
                }
            },
        )
        .collect::<Vec<NewProduct>>();

    let storage = Storage::from_vec(mock_data::STORAGE.to_vec())
        .into_iter()
        .map(
            |Storage {
                 product_id,
                 user_id,
                 drawer_id,
                 weight_grams,
                 date_in,
                 date_out,
                 ..
             }| {
                NewStorageItem {
                    product_id,
                    drawer_id,
                    weight_grams,
                    date_in,
                    user_id,
                }
            },
        )
        .collect::<Vec<NewStorageItem>>();

    let conn = &mut establish_connection(env::var("DATABASE_URL").ok());

    // let drop_query = [
    //     "DROP TABLE IF EXISTS users CASCADE;",
    //     "DROP TABLE IF EXISTS email_whitelist CASCADE;",
    //     "DROP TABLE IF EXISTS storage CASCADE;",
    //     "DROP TABLE IF EXISTS drawers CASCADE;",
    //     "DROP TABLE IF EXISTS products CASCADE;",
    //     "DROP TABLE IF EXISTS freezers CASCADE;"
    // ];
    //
    // for query in drop_query.iter()  {
    //     println!("Running command '{}'", &query);
    //     diesel::sql_query(query.to_string())
    //         .execute(conn)
    //         .unwrap();
    // };

    // println!("Performing migrations");
    //
    // run_migrations(conn).unwrap();

    let users_insert = diesel::insert_into(users_dsl::users)
        .values(&users)
        .returning(User::as_returning())
        .get_results(conn)
        .ok();
    if users_insert.is_none() {
        println!("No users have been inserted.");
    } else {
        dbg!(users_insert.unwrap());
    }

    let whitelist_insert = diesel::insert_into(whitelist_dsl::email_whitelist)
        .values(&email_whitelist)
        .returning(EmailWhitelist::as_returning())
        .get_results(conn)
        .ok();
    if whitelist_insert.is_none() {
        println!("No whitelist entries have been inserted.");
    } else {
        dbg!(whitelist_insert.unwrap());
    }

    let freezers_insert = diesel::insert_into(freezers_dsl::freezers)
        .values(&freezers)
        .returning(Freezer::as_returning())
        .get_results(conn)
        .ok();
    if freezers_insert.is_none() {
        println!("No freezers entries have been inserted.");
    } else {
        dbg!(freezers_insert.unwrap());
    }

    let drawers_insert = diesel::insert_into(drawers_dsl::drawers)
        .values(&drawers)
        .returning(Drawer::as_returning())
        .get_results(conn)
        .ok();
    if drawers_insert.is_none() {
        println!("No drawers entries have been inserted.");
    } else {
        dbg!(drawers_insert.unwrap());
    }

    let products_insert = diesel::insert_into(products_dsl::products)
        .values(&products)
        .returning(Product::as_returning())
        .get_results(conn)
        .ok();
    if products_insert.is_none() {
        println!("No products entries have been inserted.");
    } else {
        dbg!(products_insert.unwrap());
    }

    let storage_insert = diesel::insert_into(storage_dsl::storage)
        .values(&storage)
        .returning(Storage::as_returning())
        .get_results(conn)
        .ok();
    if storage_insert.is_none() {
        println!("No storage entries have been inserted.");
    } else {
        dbg!(storage_insert.unwrap());
    }
}
