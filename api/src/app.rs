use diesel_migrations::MigrationHarness;
use tokio::signal;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use std::net::SocketAddr;

use crate::core::connection::{establish_connection, MIGRATIONS};
use crate::router::router;

/// API entry point.
pub async fn app() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "api=debug,tower_http=debug,axum::rejection=trace".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Run pending migrations prior to server startup.
    let conn = &mut establish_connection(None);
    conn.run_pending_migrations(MIGRATIONS).unwrap_or_else(|_| {
        tracing::error!(target: "database_startup", "Failed to run pending migrations");
        panic!("Failed migrations.")
    });

    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    tracing::debug!("listening on {} at port {}", addr.ip(), addr.port());

    axum::serve(listener, router(None).await.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap_or_else(|err| {
            tracing::error!(target: "app_main", "Failed to build server: {}", err);
        });
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c().await.unwrap_or_else(|error| {
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
