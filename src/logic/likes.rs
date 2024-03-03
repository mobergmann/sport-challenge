use crate::database;
use crate::database::Error;
use crate::logic::AuthContext;

use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::Json;
use http::StatusCode;
use sqlx::SqlitePool;

pub async fn get_likes(
    State(pool): State<SqlitePool>,
    Path(activity_id): Path<i64>,
) -> impl IntoResponse {
    let likes = match database::likes::get(pool, &activity_id).await {
        Ok(likes) => likes,
        Err(Error::SQLX(e)) => return match e {
            sqlx::Error::RowNotFound => (StatusCode::NOT_FOUND),
            _ => (StatusCode::INTERNAL_SERVER_ERROR),
        }.into_response(),
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
    };

    (StatusCode::OK, Json(likes)).into_response()
}

pub async fn post_like(
    State(pool): State<SqlitePool>,
    auth: AuthContext,
    Path(activity_id): Path<i64>,
) -> impl IntoResponse {
    let athlete_id = auth.current_user.unwrap().id;

    let like = match database::likes::insert(pool, &athlete_id, &activity_id).await {
        Ok(like) => like,
        Err(Error::SQLX(e)) => return match e {
            sqlx::Error::RowNotFound => StatusCode::NOT_FOUND,
            sqlx::Error::Database(e) => match e.code() {
                Some(code) if code == "2067" => StatusCode::CONFLICT,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
                },
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }.into_response(),
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
    };

    (StatusCode::OK, Json(like)).into_response()
}

pub async fn delete_like(
    State(pool): State<SqlitePool>,
    auth: AuthContext,
    Path(activity_id): Path<i64>,
) -> impl IntoResponse {
    let athlete_id = auth.current_user.unwrap().id;

    let like = match database::likes::delete(pool, &athlete_id, &activity_id).await {
        Ok(like) => like,
        Err(Error::SQLX(e)) => return match e {
            sqlx::Error::RowNotFound => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }.into_response(),
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
    };

    (StatusCode::OK, Json(like)).into_response()
}
