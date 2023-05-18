mod config;

use config::Config;
use dotenv::dotenv;
use sqlx::{mysql::MySql, mysql::MySqlPoolOptions, Pool};

use tracing::info;
use tracing_subscriber;

mod components;
use components::server::Server;

pub struct AppState {
    db: Pool<MySql>,
    env: Config,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    
    tracing_subscriber::fmt::init();

    let config = &Config::init();

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
    info!("DB Connected");
    let server = Server::init(config, &pool);
    info!("Server online");
    axum::Server::bind(
        &format!("{}:{}", config.post_adress, config.post_port)
            .parse()
            .unwrap(),
    )
    .serve(server.app.into_make_service())
    .await
    .unwrap_or_else(|err| eprintln!("Failed to bind the server: {}", err));
    info!("Server bound");
}
