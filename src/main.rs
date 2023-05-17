mod config;
mod handler;
mod jwt_auth;
mod model;
mod response;
mod route;

use config::Config;
use route::create_router;

use std::sync::Arc;
use dotenv::dotenv;
use tower_http::cors::CorsLayer;
use sqlx::{mysql::MySql, Pool, mysql::MySqlPoolOptions};
use axum::http::{
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    HeaderValue, Method,
};

pub struct AppState {
    db: Pool<MySql>,
    env: Config,
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let config = Config::init();

    let pool = match MySqlPoolOptions::new()
    .max_connections(10)
    .connect(&config.database_url)
    .await
    {
        Ok(pool) => pool,
        Err(err) => {
            eprintln!("🔥 Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };

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

    println!("🚀 Server started successfully");
    axum::Server::bind(&format!("{}:{}", config.post_adress, config.post_port).parse().unwrap())
        .serve(app.into_make_service())
        .await    
        .unwrap_or_else(|err| eprintln!("Failed to bind the server: {}", err));
}