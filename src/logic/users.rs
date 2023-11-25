use crate::database;

use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::Json;
use http::StatusCode;
use sqlx::SqlitePool;
use crate::database::Error;

pub async fn get_user(
    State(pool): State<&SqlitePool>,
    Path(username): Path<String>,
) -> impl IntoResponse {
    let user = match database::user::get(pool, &username).await {
        Ok(user) => user,
        // Err(Error::ElementNotFound) => return (StatusCode::NO_CONTENT).into_response(),
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
    };

    (StatusCode::OK, Json(user)).into_response()
}

pub async fn get_user_id(
    State(pool): State<&SqlitePool>,
    Path(user_id): Path<i64>,
) -> impl IntoResponse {
    let user = match database::user::get_id(pool, user_id).await {
        Ok(user) => user,
        // Err(Error::ElementNotFound) => return (StatusCode::NO_CONTENT).into_response(),
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
    };

    (StatusCode::OK, Json(user)).into_response()
}
