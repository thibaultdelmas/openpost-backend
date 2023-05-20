use crate::{
    components::handler::auth_handlers::{login_user_handler, logout_handler},
    AppState,
};
use axum::{
    middleware,
    routing::{get, get_service, post},
    Router,
};
use std::sync::Arc;
use tower_http::services::ServeDir;

use super::{
    auth::auth,
    handler::{
        auth_handlers::register_user_handler,
        health_checker_handler,
        post_handlers::{
            create_post_handler, delete_post_handler, get_post_handler, get_post_list_handler,
        },
        user_handlers::get_me_handler,
    },
};

pub fn create_router(app_state: Arc<AppState>) -> Router {
    let auth_middleware = middleware::from_fn_with_state(app_state.clone(), auth);

    Router::new()
        .route("/api/healthchecker", get(health_checker_handler))
        .route("/api/auth/register", post(register_user_handler))
        .route("/api/auth/login", post(login_user_handler))
        .route(
            "/api/auth/logout",
            get(logout_handler).route_layer(auth_middleware.clone()),
        )
        .route(
            "/api/users/me",
            get(get_me_handler).route_layer(auth_middleware.clone()),
        )
        .route(
            "/api/posts",
            post(create_post_handler).route_layer(auth_middleware.clone()),
        )
        .route("/api/posts", get(get_post_list_handler))
        .route(
            "/api/posts/:id",
            get(get_post_handler)
                .delete(delete_post_handler)
                .route_layer(auth_middleware),
        )
        .with_state(app_state)
        .fallback_service(static_web_delivery())
}

fn static_web_delivery() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./web/")))
}
