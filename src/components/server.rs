use axum::http::{
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    HeaderValue, Method,
};
use axum::Router;
use sqlx::{MySql, Pool};
use std::sync::Arc;
use tower_http::cors::CorsLayer;

use crate::{components::route::create_router, config::Config, AppState};
pub struct Server {
    pub app: Router,
}

impl Server {
    pub fn init(config: &Config, pool: &Pool<MySql>) -> Server {
        let cors = CorsLayer::new()
            .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
            .allow_methods([Method::GET, Method::POST, Method::DELETE])
            .allow_credentials(true)
            .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

        let app = create_router(Arc::new(AppState {
            db: pool.clone(),
            env: config.clone(),
        }))
        .layer(cors);
        Server { app }
    }
}
