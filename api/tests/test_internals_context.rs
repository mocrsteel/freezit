#[macro_use]
extern crate diesel_migrations;

mod common;

use common::db::Context;

use diesel::prelude::*;

#[test]
fn context_setup_and_drop() {
    let ctx = Context::new("internals");
    assert!(true);
}

#[test]
fn second_context() {
    let ctx = Context::new("internals");
    assert!(true)
}

#[test]
fn tables() {
    let mut ctx = Context::new("internals_tables");
    let conn = &mut ctx.establish_connection();

    diesel::sql_query("SELECT * FROM products;")
        .execute(conn)
        .unwrap();

    diesel::sql_query("SELECT * FROM storage;")
        .execute(conn)
        .unwrap();

    let false_table_returns_error = diesel::sql_query("SELECT * FROM does_not_exist")
        .execute(conn)
        .is_err();

    assert!(false_table_returns_error);
}
