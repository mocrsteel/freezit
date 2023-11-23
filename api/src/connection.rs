//!
use diesel_migrations::{embed_migrations, EmbeddedMigrations};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;

use std::env;
use regex::RegexSet;

/// Required migrations for the application's database. Embedded from the diesel `migrations` folder.
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations/");

/// Database connection. Can be set using a String for end-to-end testing.
pub fn establish_connection(db_uri: Option<String>) -> PgConnection {
    let database_url = match db_uri {
        Some(uri) => uri,
        None => {
            dotenv().ok();
            env::var("DATABASE_URL").expect("DATABASE_URL must be set")
        }
    };


    // Database uri validity check
    let uri_parts = RegexSet::new([
        r"^postgres://.*",
        r"^.*//.*:.*",
        r".*@([0-9]+(\.[0-9]+){3,5}|localhost).*",
        r"^.*/[a-zA-Z0-9_]+.*",
        r".*\?connect_timeout=[0-9]+.*",
        r"^postgres://.*/postgres.?",
    ]).unwrap().matches(&database_url);

    if !uri_parts.matched(0) {
        panic!("DATABASE_URL does not contain 'postgres:// prefix.")
    }
    if !uri_parts.matched(1) {
        panic!("Both username and password required in DATABASE_URL.")
    }
    if !uri_parts.matched(2) {
        panic!("'localhost' or IP address required in DATABASE_URL.")
    }
    if !uri_parts.matched(4) {
        panic!("Timeout required in DATABASE_URL: dbname?connect_timeout=<u32>" )
    }
    // database not set but timeout is.
    if !uri_parts.matched(3) & uri_parts.matched(4) {
        panic!("Database name required in DATABASE_URL")
    }
    // connection to postgres default database not allowed
    if uri_parts.matched(5) {
        panic!("Connecting to database 'postgres' is not allowed.");
    }

    PgConnection::establish(&database_url)
        .unwrap_or_else(|err|
            match err {
                ConnectionError::BadConnection(err) => panic!("Ensure the specified database is running: {}", err),
                ConnectionError::InvalidConnectionUrl(err) => panic!("Invalid connection string: {}", err),
                _ => panic!("Error connecting to database {}", database_url),
            }
        )
}