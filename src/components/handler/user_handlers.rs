use crate::components::model::{response::FilteredUser, structure::User};
use axum::{http::StatusCode, response::IntoResponse, Extension, Json};

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
