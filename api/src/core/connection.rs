//! Database connection.
use diesel_migrations::{embed_migrations, EmbeddedMigrations};
use diesel::pg::PgConnection;
use diesel::prelude::*;
#[cfg(not(test))]
use dotenvy::dotenv;

#[cfg(not(test))]
use std::env;
use regex::RegexSet;

/// Required migrations for the application's database. Embedded from the diesel `migrations` folder.
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations/");

/// Database connection. Can be set using a String for end-to-end testing.
pub fn establish_connection(db_uri: Option<String>) -> PgConnection {
    #[cfg(not(test))]
    let database_url = db_uri.unwrap_or_else(|| {
        dotenv().ok();
        env::var("DATABASE_URL").expect("DATABASE_URL must be set")
    });
    #[cfg(test)]
    let database_url = db_uri.unwrap_or_else(|| {
        panic!("DATABASE_URL must be set")
    });

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
#[cfg(test)]
mod uri_parsing {
    use super::*;
    use std::env;
    #[test]
    #[should_panic(expected = "DATABASE_URL must be set")]
    fn database_url_not_set() {
        if env::var("DATABASE_URL").is_ok() {
            env::remove_var("DATABASE_URL");
        }
        assert!(env::var("DATABASE_URL").is_err());

        establish_connection(None);
    }
    #[test]
    #[should_panic(expected = "DATABASE_URL does not contain 'postgres:// prefix.")]
    fn postgres_prefix_missing() {
        establish_connection(Some(String::from("http://postgres_user:postgres_pw@localhost/db_name?connect_timeout=20")));
    }

    #[test]
    #[should_panic(expected = "Both username and password required in DATABASE_URL.")]
    fn username_password_not_set() {
        establish_connection(Some(String::from("postgres://postgres_pw@localhost/freezit?connect_timeout=20")));
    }

    #[test]
    #[should_panic(expected = "'localhost' or IP address required in DATABASE_URL.")]
    fn host_address_not_set_correctly() {
        establish_connection(Some(String::from("postgres://postgres_user:postgres_pw@other_host/freezit?connect_timeout=20")));
    }

    #[test]
    #[should_panic(expected = "Timeout required in DATABASE_URL: dbname?connect_timeout=<u32>" )]
    fn timeout_not_set() {
        establish_connection(Some(String::from("postgres://postgres_user:postgres_pw@localhost/freezit")));
    }

    #[test]
    #[should_panic(expected = "Connecting to database 'postgres' is not allowed.")]
    fn db_set_to_postgres() {
        establish_connection(Some(String::from("postgres://postgres_user:postgres_pw@localhost/postgres?connect_timeout=20")));
    }
}