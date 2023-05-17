#[derive(Debug, Clone, Default)]
pub struct Config {
    pub database_url: String,
    pub post_adress: String,
    pub post_port: String,
    pub jwt_secret: String,
    pub jwt_expires_in: String,
    pub jwt_maxage: i32,
}

impl Config {
    pub fn init() -> Config {
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let post_adress = std::env::var("POST_ADRESS").unwrap_or("0.0.0.0".to_string());
        let post_port = std::env::var("POST_PORT").unwrap_or("8000".to_string());
        let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
        let jwt_expires_in = std::env::var("JWT_EXPIRED_IN").expect("JWT_EXPIRED_IN must be set");
        let jwt_maxage = std::env::var("JWT_MAXAGE").expect("JWT_MAXAGE must be set");
        Config {
            database_url,
            post_adress,
            post_port,
            jwt_secret,
            jwt_expires_in,
            jwt_maxage: jwt_maxage.parse::<i32>().unwrap(),
        }
    }
}