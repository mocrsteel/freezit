//! Endpoint `/api/users`.

use axum::extract::State;
use axum::http::{HeaderMap, StatusCode};
use uuid::Uuid;
use crate::router::AppState;
use crate::models::User;

pub fn get_all_users(
    header_map: HeaderMap,
    State(state): State<AppState>,
) -> Result<Vec<User>, (StatusCode, String)> {

    Ok(vec!(User {id: Uuid::new_v4(), name: String::from("Test"), email: String::from("Email"), permissions: 0i32 },))
}