use std::sync::Arc;

use axum::{
    extract::State,
    http::{header, Request, StatusCode},
    middleware::Next,
    response::IntoResponse,
    Json,
};

use axum_extra::extract::cookie::CookieJar;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::{components::model::structure::User, AppState};

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub status: &'static str,
    pub message: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: String,
    pub iat: usize,
    pub exp: usize,
}

pub async fn auth<B>(
    cookie_jar: CookieJar,
    State(data): State<Arc<AppState>>,
    mut req: Request<B>,
    next: Next<B>,
) -> Result<impl IntoResponse, (StatusCode, Json<ErrorResponse>)> {
    let token = cookie_jar
        .get("token")
        .map(|cookie| cookie.value().to_string())
        .or_else(|| {
            req.headers()
                .get(header::AUTHORIZATION)
                .and_then(|auth_header| auth_header.to_str().ok())
                .and_then(|auth_value| auth_value.strip_prefix("Bearer ").map(str::to_owned))
        });

    let token = token.ok_or_else(|| {
        let json_error = ErrorResponse {
            status: "fail",
            message: "You are not logged in, please provide token".to_string(),
        };
        (StatusCode::UNAUTHORIZED, Json(json_error))
    })?;

    //let token = "{'token':".to_string() + &token.to_string() + "}";
    println!("{}", token);

    let claims = decode::<TokenClaims>(
        &token,
        &DecodingKey::from_secret(data.env.jwt_secret.as_ref()),
        &Validation::new(Algorithm::HS256),
    )
    .map_err(|e| {
        let json_error = ErrorResponse {
            status: "fail",
            message: "Invalid token provided".to_string() + &e.to_string() + &token.to_string(),
        };
        (StatusCode::UNAUTHORIZED, Json(json_error))
    })?
    .claims;

    let user_id = uuid::Uuid::parse_str(&claims.sub).map_err(|e| {
        let json_error = ErrorResponse {
            status: "fail",
            message: "Invalid token".to_string() + &e.to_string(),
        };
        (StatusCode::UNAUTHORIZED, Json(json_error))
    })?;

    let user = sqlx::query_as!(User, "SELECT * FROM `user` WHERE user_id = ?", user_id)
        .fetch_optional(&data.db)
        .await
        .map_err(|e| {
            let json_error = ErrorResponse {
                status: "fail",
                message: format!("Error fetching user from database: {}", e),
            };
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json_error))
        })?
        .ok_or_else(|| {
            let json_error = ErrorResponse {
                status: "fail",
                message: "The user belonging to this token no longer exists".to_string(),
            };
            (StatusCode::UNAUTHORIZED, Json(json_error))
        })?;

    req.extensions_mut().insert(user);
    Ok(next.run(req).await)
}

pub fn generate_auth_cookie(user: &User, State(data): State<Arc<AppState>>) -> String {
    let now = chrono::Utc::now();
    let iat = now.timestamp() as usize;
    let exp = (now + chrono::Duration::minutes(60)).timestamp() as usize;

    let claims: TokenClaims = TokenClaims {
        sub: user.user_id_text.clone().expect("No user_id str rep"),
        exp,
        iat,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(data.env.jwt_secret.as_ref()),
    )
    .unwrap();

    token
}
