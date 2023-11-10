#[macro_use]
extern crate diesel_migrations;

mod common;

use common::db::Context;

use diesel::prelude::*;
use log::{info, warn};

mod context {
    use super::*;

    #[test]
    fn context_setup_and_drop() {
        warn!("this could get hairy");
        let _ctx = Context::new("internals");
        info!("We made it!");
        // assert!(true)
    }

    #[test]
    fn second_context() {
        let _ctx = Context::new("internals");
        // assert!(true)
    }
}

mod migrations {
    use super::*;
        mod tables {
            use super::*;
            use test_log::test;

            static CTX: &str = "migrations_tables";

            #[test]
            fn correct_tables_present() {
                let mut ctx = Context::new(CTX);
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
    mod columns {
        use super::*;
        use test_log::test;

        static CTX: &str = "columns";
        #[test]
        fn product_has_correct_columns() {
            let mut ctx = Context::new(CTX);
            let conn = &mut ctx.establish_connection();

            let product_result = diesel::sql_query("SELECT product_id, name, expiration_months FROM products;")
                .execute(conn)
                .is_ok();

            assert!(product_result);
        }

        #[test]
        fn storage_has_correct_columns() {
            let mut ctx = Context::new(CTX);
            let conn = &mut ctx.establish_connection();

            let storage_result = diesel::sql_query("SELECT storage_id, product_id, weight_grams, date_in, date_out, available, drawer_id FROM storage;")
                .execute(conn)
                .is_ok();

            assert!(storage_result);
        }

        # [test]
        fn freezers_has_correct_columns() {
            let mut ctx = Context::new(CTX);
            let conn = &mut ctx.establish_connection();

            let res = diesel::sql_query("SELECT freezer_id, name FROM freezers;")
                .execute(conn)
                .is_ok();

            assert!(res);
        }

        #[test]
        fn drawers_has_correct_columns() {
            let mut ctx = Context::new(CTX);
            let conn = &mut ctx.establish_connection();

            let res = diesel::sql_query("SELECT drawer_id, name, freezer_id FROM drawers;")
                .execute(conn)
                .is_ok();

            assert!(res);
        }
    }
}
