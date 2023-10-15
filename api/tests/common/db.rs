use std::error::Error;
use std::fmt::format;
use std::sync::atomic::Ordering;

use diesel::{Connection, PgConnection, RunQueryDsl};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

use dotenvy::dotenv;
use log::{info, warn, error, debug};

use super::DB_COUNT;

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

        let base_url = std::env::var("DATABASE_BASE_URL").expect("DATABASE_BASE_URL expected for integration testing.");

        let db_num = DB_COUNT.fetch_add(1, Ordering::SeqCst);
        let db_name = format!("db_{}_{}", db_num, ctx);
        let postgres_url = format!("{}/postgres", base_url);

        debug!(target:"interation_tests > Context", "Creating database {} at postgres instance {}", db_name, postgres_url);
        let conn = & mut PgConnection::establish(&postgres_url)
            .expect("Cannot connect to postgres database.");

        let query = diesel::sql_query(format!("CREATE DATABASE {}", db_name.as_str()));
        query
            .execute(conn)
            .expect(format!("Could not create database {}", db_name).as_str());

        debug!(target: LOG_TARGET, "Database {} successfully created", db_name);

        let conn = & mut PgConnection::establish(format!("{}/{}", base_url, db_name).as_str())
            .expect(format!("Error connecting to newly created database {}", db_name).as_str());

        conn.run_pending_migrations(MIGRATIONS).unwrap();

        return Self {
            base_url,
            db_name
        }
    }
    pub fn establish_connection(&mut self) -> PgConnection {
        PgConnection::establish(format!("{}/{}", self.base_url, self.db_name).as_str())
            .expect(format!("Could not connect to database {}", self.db_name).as_str())
    }
}
impl Drop for Context {
    fn drop(&mut self) {
        debug!(target: LOG_TARGET, "Preparing to drop database {} at postgres instance {}", self.db_name, self.base_url);

        let postgres_url = format!("{}/postgres", self.base_url);
        let conn = &mut PgConnection::establish(format!("{}/postgres", self.base_url).as_str())
            .expect("Could not connect to the postgres database.");

        debug!(target: LOG_TARGET, "Disconnecting users from {}", self.db_name);
        let disconnect_users = format!(
            "SELECT pg_terminate_backend(pg_stat_activity.pid)
            FROM pg_stat_activity
            WHERE pg_stat_activity.datname='{}';",
            self.db_name
        );
        diesel::sql_query(disconnect_users)
            .execute(conn)
            .unwrap();
        let query = diesel::sql_query(format!("DROP DATABASE {}", self.db_name).as_str());

        debug!(target: LOG_TARGET, "Dropping database {}", self.db_name);
        query
            .execute(conn)
            .expect(format!("Could not drop database {}", self.db_name).as_str());
    }
}
