use std::sync::Arc;

use argon2::{password_hash::SaltString, Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use axum::{
    extract::{Path, Query, State},
    http::{header, Response, StatusCode},
    response::IntoResponse,
    Extension, Json,
};
use axum_extra::extract::cookie::{Cookie, SameSite};
use jsonwebtoken::{encode, EncodingKey, Header};
use rand_core::OsRng;
use serde_json::json;

use crate::{
    components::response::PostResponse,
    components::{
        model::{
            CreatePostSchema, FilterOptions, LoginUserSchema, Post, RegisterUserSchema,
            TokenClaims, User,
        },
        response::FilteredUser,
    },
    AppState,
};

pub async fn health_checker_handler() -> impl IntoResponse {
    const MESSAGE: &str = "Alive";

    let json_response = serde_json::json!({
        "status": "success",
        "message": MESSAGE
    });

    Json(json_response)
}

pub async fn register_user_handler(
    State(data): State<Arc<AppState>>,
    Json(body): Json<RegisterUserSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let user_exists: Option<bool> =
        sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM user WHERE email = $1)")
            .bind(body.email.to_owned().to_ascii_lowercase())
            .fetch_one(&data.db)
            .await
            .map_err(|e| {
                let error_response = serde_json::json!({
                    "status": "fail",
                    "message": format!("Database error: {}", e),
                });
                (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
            })?;

    if let Some(exists) = user_exists {
        if exists {
            let error_response = serde_json::json!({
                "status": "fail",
                "message": "User with that email already exists",
            });
            return Err((StatusCode::CONFLICT, Json(error_response)));
        }
    }

    let salt = SaltString::generate(&mut OsRng);
    let hashed_password = Argon2::default()
        .hash_password(body.password.as_bytes(), &salt)
        .map_err(|e| {
            let error_response = serde_json::json!({
                "status": "fail",
                "message": format!("Error while hashing password: {}", e),
            });
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
        })
        .map(|hash| hash.to_string())?;

    let user = sqlx::query_as!(
        User,
        "INSERT INTO `user` (user_name, email, hash_pass) VALUES (?, ?, ?);",
        body.name.to_string(),
        body.email.to_string().to_ascii_lowercase(),
        hashed_password
    )
    .fetch_one(&data.db)
    .await
    .map_err(|e| {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": format!("Database error: {}", e),
        });
        (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
    })?;

    let user_response = serde_json::json!({"status": "success","data": serde_json::json!({
        "user": FilteredUser::filter_user_record(&user)
    })});

    Ok(Json(user_response))
}

pub async fn login_user_handler(
    State(data): State<Arc<AppState>>,
    Json(body): Json<LoginUserSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let user = sqlx::query_as!(
        User,
        "SELECT * FROM `user` WHERE email = ?",
        body.email.to_ascii_lowercase()
    )
    .fetch_optional(&data.db)
    .await
    .map_err(|e| {
        let error_response = serde_json::json!({
            "status": "error",
            "message": format!("Database error: {}", e),
        });
        (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
    })?
    .ok_or_else(|| {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": "Invalid email or password",
        });
        (StatusCode::BAD_REQUEST, Json(error_response))
    })?;

    let is_valid = match PasswordHash::new(&user.hash_pass) {
        Ok(parsed_hash) => Argon2::default()
            .verify_password(body.password.as_bytes(), &parsed_hash)
            .map_or(false, |_| true),
        Err(_) => false,
    };

    if !is_valid {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": "Invalid email or password"
        });
        return Err((StatusCode::BAD_REQUEST, Json(error_response)));
    }

    let now = chrono::Utc::now();
    let iat = now.timestamp() as usize;
    let exp = (now + chrono::Duration::minutes(60)).timestamp() as usize;
    let claims: TokenClaims = TokenClaims {
        sub: user.user_id,
        exp,
        iat,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(data.env.jwt_secret.as_ref()),
    )
    .unwrap();

    let cookie = Cookie::build("token", token.to_owned())
        .path("/")
        .max_age(time::Duration::hours(1))
        .same_site(SameSite::Lax)
        .http_only(true)
        .finish();

    let mut response = Response::new(json!({"status": "success", "token": token}).to_string());
    response
        .headers_mut()
        .insert(header::SET_COOKIE, cookie.to_string().parse().unwrap());
    Ok(response)
}

pub async fn logout_handler() -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let cookie = Cookie::build("token", "")
        .path("/")
        .max_age(time::Duration::hours(-1))
        .same_site(SameSite::Lax)
        .http_only(true)
        .finish();

    let mut response = Response::new(json!({"status": "success"}).to_string());
    response
        .headers_mut()
        .insert(header::SET_COOKIE, cookie.to_string().parse().unwrap());
    Ok(response)
}

pub async fn get_me_handler(
    Extension(user): Extension<User>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let json_response = serde_json::json!({
        "status":  "success",
        "data": serde_json::json!({
            "user": FilteredUser::filter_user_record_from_user(&user)
        })
    });

    Ok(Json(json_response))
}

pub async fn get_post_list_handler(
    opts: Option<Query<FilterOptions>>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let Query(opts) = opts.unwrap_or_default();

    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    let posts = sqlx::query_as!(
        Post,
        "SELECT * FROM post ORDER by id LIMIT ? OFFSET ?",
        limit as i32,
        offset as i32
    )
    .fetch_all(&data.db)
    .await
    .map_err(|e| {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": format!("Database error: {}", e),
        });
        (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
    })?;

    let post_responses: Vec<PostResponse> = posts
        .iter()
        .map(|post: &Post| -> PostResponse { PostResponse::filter_db_record(&post) })
        .collect();

    let json_response = serde_json::json!({
        "status": "success",
        "results": post_responses.len(),
        "posts": post_responses
    });

    Ok(Json(json_response))
}

pub async fn create_post_handler(
    State(data): State<Arc<AppState>>,
    Json(body): Json<CreatePostSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result = sqlx::query(r#"INSERT INTO post (title,content) VALUES (?, ?)"#)
        .bind(body.title.to_string())
        .bind(body.content.to_string())
        .execute(&data.db)
        .await
        .map_err(|err: sqlx::Error| err.to_string());

    if let Err(err) = query_result {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"status": "error","message": format!("{:?}", err)})),
        ));
    }

    let post = sqlx::query_as!(Post, r#"SELECT * FROM post WHERE title = ?"#, body.title)
        .fetch_one(&data.db)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"status": "error","message": format!("{:?}", e)})),
            )
        })?;

    let post_response = serde_json::json!({"status": "success","data": serde_json::json!({
        "post": PostResponse::filter_db_record(&post)
    })});

    Ok(Json(post_response))
}

pub async fn get_post_handler(
    Path(id): Path<uuid::Uuid>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let post = sqlx::query_as!(Post, r#"SELECT * FROM post WHERE title = ?"#, id)
        .fetch_one(&data.db)
        .await;

    match post {
        Ok(post) => {
            let post_response = serde_json::json!({"status": "success","data": serde_json::json!({
                "post": PostResponse::filter_db_record(&post)
            })});

            Ok(Json([post_response]))
        }
        Err(sqlx::Error::RowNotFound) => {
            let error_response = serde_json::json!({
                "status": "fail",
                "message": format!("Note with ID: {} not found", id)
            });
            Err((StatusCode::NOT_FOUND, Json(error_response)))
        }
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"status": "error","message": format!("{:?}", e)})),
        )),
    }
}

pub async fn delete_post_handler(
    Path(id): Path<uuid::Uuid>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result = sqlx::query!(r#"DELETE FROM post WHERE id = ?"#, id.to_string())
        .execute(&data.db)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"status": "error","message": format!("{:?}", e)})),
            )
        })?;

    if query_result.rows_affected() == 0 {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": format!("Note with ID: {} not found", id)
        });
        return Err((StatusCode::NOT_FOUND, Json(error_response)));
    }

    Ok(StatusCode::NO_CONTENT)
}
