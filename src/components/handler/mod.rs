use axum::{response::IntoResponse, Json};

pub mod auth_handlers;
pub mod post_handlers;
pub mod user_handlers;

pub async fn health_checker_handler() -> impl IntoResponse {
    const MESSAGE: &str = "Alive";

    let json_response = serde_json::json!({
        "status": "success",
        "message": MESSAGE
    });

    Json(json_response)
}
