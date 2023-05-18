use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::Row;

use crate::components::model::{Post, User};

#[derive(Debug, Serialize)]
pub struct FilteredUser {
    pub user_id: String,
    pub name: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl FilteredUser {
    pub fn filter_user_record(user: &sqlx::mysql::MySqlRow) -> FilteredUser {
        FilteredUser {
            user_id: user.get::<String, &str>("user_id"),
            email: user.get::<String, &str>("email"),
            name: user.get::<String, &str>("user_name"),
            created_at: user.get::<chrono::DateTime<chrono::Utc>, &str>("created_at"),
            updated_at: user.get::<chrono::DateTime<chrono::Utc>, &str>("updated_at"),
        }
    }

    pub fn filter_user_record_from_user(user: &User) -> FilteredUser {
        FilteredUser {
            user_id: user.user_id.to_string(),
            email: user.email.to_owned(),
            name: user.user_name.to_owned(),
            created_at: user.created_at.unwrap(),
            updated_at: user.updated_at.unwrap(),
        }
    }
}

#[derive(Serialize, Debug)]
pub struct UserData {
    pub user: FilteredUser,
}

#[derive(Serialize, Debug)]
pub struct UserResponse {
    pub status: String,
    pub data: UserData,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PostResponse {
    pub id: String,
    pub title: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl PostResponse {
    pub fn filter_db_record(post: &Post) -> PostResponse {
        PostResponse {
            id: post.id.to_owned(),
            title: post.title.to_owned(),
            content: post.content.to_owned(),
            created_at: post.created_at.unwrap(),
            updated_at: post.updated_at.unwrap(),
        }
    }
}
