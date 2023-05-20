pub use std::sync::Arc;

pub use argon2::{password_hash::SaltString, Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
pub use axum::{
    extract::{Path, Query, State},
    http::{header, Response, StatusCode},
    response::IntoResponse,
    Extension, Json,
};
pub use axum_extra::extract::cookie::{Cookie, SameSite};
pub use rand_core::OsRng;
pub use serde_json::json;

pub use crate::{
    components::response::PostResponse,
    components::{
        model::{CreatePostSchema, FilterOptions, LoginUserSchema, Post, RegisterUserSchema, User},
        response::FilteredUser,
    },
    AppState,
};

pub mod auth_handlers;
pub mod user_handlers;
pub mod post_handlers;

use super::jwt_auth::generate_auth_cookie;

pub async fn health_checker_handler() -> impl IntoResponse {
    const MESSAGE: &str = "Alive";

    let json_response = serde_json::json!({
        "status": "success",
        "message": MESSAGE
    });

    Json(json_response)
}
