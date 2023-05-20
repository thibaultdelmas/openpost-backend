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
