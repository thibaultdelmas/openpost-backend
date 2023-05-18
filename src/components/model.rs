use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow, Clone)]
pub struct User {
    pub user_id: String,
    pub user_name: String,
    pub email: String,
    pub hash_pass: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: String,
    pub iat: usize,
    pub exp: usize,
}

#[derive(Debug, Deserialize)]
pub struct RegisterUserSchema {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginUserSchema {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct Post {
    pub id: String,
    pub title: String,
    pub content: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Deserialize, Debug, Default)]
pub struct FilterOptions {
    pub page: Option<usize>,
    pub limit: Option<usize>,
}

#[derive(Deserialize, Debug)]
pub struct ParamOptions {
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreatePostSchema {
    pub title: String,
    pub content: String,
}
