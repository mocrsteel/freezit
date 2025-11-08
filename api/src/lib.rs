//! Main lib to make all modules available to the binaries and to the crate internally.
mod core;
mod app;
#[cfg(any(test, feature = "seed"))]
mod mock_data;
mod models;
mod router;
mod routes;
mod schema;
mod utils;

#[cfg(test)]
mod tests;

pub use app::app;

#[cfg(any(test, feature = "seed"))]
pub use utils::seed::seed;

