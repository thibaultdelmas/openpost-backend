mod config;

use config::Config;
use dotenv::dotenv;
use sqlx::{mysql::MySql, mysql::MySqlPoolOptions, Pool};

mod components;
use components::server::Server;

pub struct AppState {
    db: Pool<MySql>,
    env: Config,
}

#[tokio::main]
async fn main() {
    dotenv().ok();

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

    let server = Server::init(config, &pool);

    axum::Server::bind(
        &format!("{}:{}", config.post_adress, config.post_port)
            .parse()
            .unwrap(),
    )
    .serve(server.app.into_make_service())
    .await
    .unwrap_or_else(|err| eprintln!("Failed to bind the server: {}", err));

}
