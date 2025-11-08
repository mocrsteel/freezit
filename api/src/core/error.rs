//! Error handlers.
//!
//! Needs improvement to forward errors coming from the database (e.g. unique key violation etc).
//! Idea is to create an error enum per API endpoint / database table or for specific operations.
//! This should allow pushing a clear and concise error to the frontend.
//!
//! Example of implementation idea as mentioned above:
//!
//! ```no_run
//!     use axum::http::StatusCode;
//!
//!     pub enum ProductError {
//!         Duplicate,
//!         NotFound,
//!     }
//!     impl ProductError {
//!         pub fn to_string(self) -> String {
//!             match self {
//!                 Self::Duplicate => String::from("This product name already exists."),
//!                 Self::NotFound => String::from("Product could not be found.")
//!             }
//!         }
//!     }
//!     pub fn product_error<E>(err: ProductError) -> (StatusCode, String) {
//!         (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
//!     }
//! ```

use axum::http::StatusCode;
use thiserror::Error;

/// Wrapper for internal database connector errors, returning [StatusCode] 500 with the stringyfied
/// database connector error to be consumed by the frontend.
pub fn internal_error<E>(err: E) -> (StatusCode, String)
    where
        E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}

/// Representation of Api specific errors with nicer information for the specific cases.
#[derive(Debug, Error)]
pub enum ApiError {
    /// Error linked to authorization: JWT initializing.
    #[error("API authorization error: {0}")]
    Authorization(String),
    /// Error linked to incomplete or invalid request authorization headers.
    #[error("Invalid authorization header: expected {expected:?}, found {found:?}")]
    AuthorizationHeader {
        /// Expected value.
        expected: String,
        /// Value that was found.
        found: String,
    },
    /// Errors linked to database interactions.
    #[error("Database error: {0}")]
    Database(String),
    /// Json Web Token decode error.
    #[error("JWT decode error: {0}")]
    JWTDecode(String),
}