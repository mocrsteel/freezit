use std::net::SocketAddr;

use diesel_migrations::MigrationHarness;
use tokio::signal;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use api::app;
use api::connection::{establish_connection, MIGRATIONS};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "api=debug,tower_http=debug,axum::rejection=trace".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();


    // Run pending migrations prior to server startup.
    let conn = &mut establish_connection();
    conn.run_pending_migrations(MIGRATIONS)
        .unwrap_or_else(|_| {
            tracing::error!(target: "database_startup", "Failed to run pending migrations");
            panic!("Failed migrations.")
        });

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::debug!("listening on {} at port {}", addr.ip(), addr.port());

    hyper::Server::bind(&addr)
        .serve(app().await.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap_or_else(|err| {
            tracing::error!(target: "app_main", "Failed to build server: {}", err);
        });
    // Code references for database interactions with Diesel, delete later.

    // use api::schema::products::dsl::*;
    //
    // let conn = &mut establish_connection();
    //
    // // let new_products: Vec<NewProduct> = vec![
    // //     NewProduct {
    // //         name: "Brocoli",
    // //         expiration_months: Some(16)
    // //     },
    // //     NewProduct {
    // //         name: "Asperges",
    // //         expiration_months: Some(16)
    // //     }
    // // ];
    // // diesel::insert_into(products)
    // //     .values(new_products)
    // //     .returning(Product::as_returning())
    // //     .get_result(conn)
    // //     .expect("Error saving new product");
    // fn get_product_id(conn: &mut PgConnection, query_name: &str) -> Result<Option<i32>, Error> {
    //     products
    //         .select(product_id)
    //         .filter(name.eq(query_name))
    //         .get_result(conn)
    //         .optional()
    // }
    //
    // let results = products
    //     .select(Product::as_select())
    //     .load(conn)
    //     .expect("Error loading products");
    // for product in results {
    //     println!("{} expires after {} months", product.name, product.expiration_months);
    // }
    //
    // if let Ok(Some(test_name)) = get_product_id(conn, "Brocoli") {
    //     println!("{}", test_name);
    // } else {
    //     println!("Could not find Brocoli");
    // }
    // if let Ok(Some(false_name)) = get_product_id(conn, "Non existent") {
    //     println!("{}", false_name);
    // } else {
    //     println!("Could not find 'Non existent'");
    // }
    //
    // if let Ok(Some(new_product_id)) = get_product_id(conn, "Brocoli") {
    //     let new_storage_item = NewStorageItem {
    //         product_id: new_product_id,
    //         weight_grams: 525.2,
    //         date_in: Local::now().date_naive(),
    //         available: true,
    //     };
    //     diesel::insert_into(storage)
    //         .values(new_storage_item)
    //         .returning(Storage::as_returning())
    //         .get_result(conn)
    //         .expect("Could not add new storage item!");
    // } else {
    //     println!("Could not find the product in the database.");
    // }
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .unwrap_or_else(|error| {
                tracing::error!(target: "app_main", "Failed to install Ctrl+C handler: {}", error);
                panic!("Failed shutdown setup")
            });
    };

    #[cfg(unix)]
        let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .unwrap_or_else(|err| {
                tracing::error!(target: "shutdown_setup", "Failed to install signal handler: {}", err);
                panic!("Failed shutdown setup.")
            })
            .recv()
            .await;
    };

    #[cfg(not(unix))]
        let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {}
    }

    tracing::info!(target: "shutdown_event", "signal received starting graceful shutdown.");
}