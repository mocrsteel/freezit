#![allow(dead_code)]

use std::sync::atomic::Ordering;

use diesel::prelude::*;
use diesel::{Connection, PgConnection, RunQueryDsl};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

use chrono::prelude::*;
use dotenvy::dotenv;
use log::{debug, error, info};

use api::models::{NewFreezer, NewProduct, NewStorageItem, NewDrawer, Drawer, Freezer, Product, Storage};

use super::{DB_COUNT, db_data};

static LOG_TARGET: &str = "integration_tests > Context";
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

#[derive(Debug)]
pub struct Context {
    base_url: String,
    db_name: String,
}

impl Context {
    pub fn new(ctx: &str) -> Context {
        info!(target: LOG_TARGET, "Setting up context for {ctx}");
        dotenv().ok();

        let base_url = std::env::var("DATABASE_BASE_URL")
            .expect("DATABASE_BASE_URL expected for integration testing.");

        let db_num = DB_COUNT.fetch_add(1, Ordering::SeqCst);
        let db_name = format!("db_{}_{}", db_num, ctx);
        let postgres_url = format!("{}/postgres?connect_timeout=5", base_url);

        debug!(target:"interaction_tests > Context", "Creating database {} at postgres instance {}", db_name, postgres_url);
        let conn = &mut PgConnection::establish(&postgres_url)
            .expect("Cannot connect to postgres database.");

        let query = diesel::sql_query(format!("CREATE DATABASE {}", db_name.as_str()));
        query.execute(conn).unwrap_or_else(|err| {
            error!("Error creating database {}: {}", db_name, err);
            panic!("Could not create database {}", db_name)
        });

        info!(target: LOG_TARGET, "Database {} successfully created", db_name);

        let conn = &mut PgConnection::establish(format!("{}/{}", base_url, db_name).as_str())
            .unwrap_or_else(|err| {
                error!(
                    "Error connecting to database '{}' upon initialization: {}",
                    db_name, err
                );
                panic!("Error connecting to newly created database {}", db_name)
            });

        debug!("Performing migrations");
        conn.run_pending_migrations(MIGRATIONS)
            .unwrap_or_else(|err| {
                error!(
                    "Could not perform migrations on testing database '{}': {}",
                    db_name, err
                );
                panic!("Failed migrations in database '{}'", db_name)
            });

        debug!("Priming database.");
        Self::feed_database(conn, &db_name);

        info!("Successfully created and primed database '{}'", db_name);

        Self { base_url, db_name }
    }
    pub fn establish_connection(&mut self) -> PgConnection {
        PgConnection::establish(format!("{}/{}?connect_timeout=5", self.base_url, self.db_name).as_str())
            .unwrap_or_else(|_| panic!("Could not connect to database {}", self.db_name))
    }
    pub fn database_url(&self) -> String {
        format!("{}/{}?connect_timeout=5", self.base_url, self.db_name)
    }

    fn feed_database(conn: &mut PgConnection, db_name: &str) {
        // Data preparation prior to feeding it to the context database.
        let freezers_feed: Vec<NewFreezer> = db_data::FREEZERS
            .into_iter()
            .map(|(_id, name)| {
                NewFreezer {
                    name: String::from(name),
                }
            }).collect();
        let drawers_feed: Vec<NewDrawer> = db_data::DRAWERS
            .into_iter()
            .map(|(_id, name, freezer_id)| {
                NewDrawer {
                    name: String::from(name),
                    freezer_id,
                }
            }).collect();
        let product_feed: Vec<NewProduct> = db_data::PRODUCTS
            .into_iter()
            .map(|(_id, name, expiration_months)| {
                NewProduct {
                    name: String::from(name),
                    expiration_months: Some(expiration_months),
                }
            }).collect();
        let storage_feed: Vec<NewStorageItem> = db_data::STORAGE
            .into_iter()
            .map(|(_id, prod_id, wt_grams, dt_in, av, draw_id)| {
                NewStorageItem {
                    product_id: prod_id,
                    weight_grams: wt_grams,
                    date_in: NaiveDate::parse_from_str(dt_in, "%Y-%m-%d").unwrap(),
                    available: av,
                    drawer_id: draw_id,
                }
            }).collect();

        use api::schema::drawers::dsl as draw;
        use api::schema::freezers::dsl as freez;
        use api::schema::products::dsl as prod;
        use api::schema::storage::dsl as stor;

        // let conn = &mut self.establish_connection();
        diesel::insert_into(freez::freezers)
            .values(freezers_feed)
            .returning(Freezer::as_returning())
            .get_result(conn)
            .unwrap_or_else(|err| {
                error!("Error loading freezers in '{}': {}", db_name, err);
                panic!("Error loading freezers into database {}", db_name)
            });

        diesel::insert_into(draw::drawers)
            .values(drawers_feed)
            .returning(Drawer::as_returning())
            .get_result(conn)
            .unwrap_or_else(|err| {
                error!("Error loading drawers in '{}': {}", db_name, err);
                panic!("Error loading drawers into database {}", db_name)
            });

        diesel::insert_into(prod::products)
            .values(product_feed)
            .returning(Product::as_returning())
            .get_results(conn)
            .unwrap_or_else(|err| {
                error!("Error loading products in '{}': {}", db_name, err);
                panic!("Error loading products into database {}", db_name)
            });

        diesel::insert_into(stor::storage)
            .values(storage_feed)
            .returning(Storage::as_returning())
            .get_results(conn)
            .unwrap_or_else(|err| {
                error!("Error loading storage items in '{}' {}", db_name, err);
                panic!("Error loading storage items into database {}", db_name)
            });
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        debug!(target: LOG_TARGET, "Preparing to drop database {} at postgres instance {}", self.db_name, self.base_url);

        let conn = &mut PgConnection::establish(format!("{}/postgres", self.base_url).as_str())
            .expect("Could not connect to the postgres database.");

        debug!(target: LOG_TARGET, "Disconnecting users from {}", self.db_name);
        let disconnect_users = format!(
            "SELECT pg_terminate_backend(pg_stat_activity.pid)
            FROM pg_stat_activity
            WHERE pg_stat_activity.datname='{}';",
            self.db_name
        );
        diesel::sql_query(disconnect_users).execute(conn).unwrap();
        let query = diesel::sql_query(format!("DROP DATABASE {}", self.db_name).as_str());

        debug!(target: LOG_TARGET, "Dropping database {}", self.db_name);
        query
            .execute(conn)
            .unwrap_or_else(|_| panic!("Could not drop database {}", self.db_name));
    }
}
