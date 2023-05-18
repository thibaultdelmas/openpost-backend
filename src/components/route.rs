use crate::{components::handler::*, components::jwt_auth::auth, AppState};
use axum::{
    middleware,
    routing::{get, post},
    Router,
};
use std::sync::Arc;

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
}
