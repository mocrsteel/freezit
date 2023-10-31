#[macro_use]
extern crate diesel_migrations;

mod common;

use common::db::Context;

use diesel::prelude::*;

mod context {
    use super::*;
    use test_log::test;
    #[test]
    fn context_setup_and_drop() {
        let _ctx = Context::new("internals");
        // assert!(true)
    }

    #[test]
    fn second_context() {
        let _ctx = Context::new("internals");
        // assert!(true)
    }
}

mod tables {
    use super::*;
    use test_log::test;

    #[test]
    fn correct_tables_present() {
        let mut ctx = Context::new("internals_tables");
        let conn = &mut ctx.establish_connection();

        diesel::sql_query("SELECT * FROM products;")
            .execute(conn)
            .unwrap();

        diesel::sql_query("SELECT * FROM storage;")
            .execute(conn)
            .unwrap();

        diesel::sql_query("SELECT * FROM freezers;")
            .execute(conn)
            .unwrap();

        diesel::sql_query("SELECT * FROM drawers;")
            .execute(conn)
            .unwrap();

        let false_table_returns_error = diesel::sql_query("SELECT * FROM does_not_exist")
            .execute(conn)
            .is_err();

        assert!(false_table_returns_error);
    }
}
