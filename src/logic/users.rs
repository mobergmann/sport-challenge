use crate::database;
use crate::database::Error;
use crate::account::User;

use axum::extract::Path;
use axum::response::IntoResponse;
use axum::Json;
use http::StatusCode;

pub async fn get_user(Path(username): Path<String>) -> impl IntoResponse {
    let user = match database::get_user(&username).await {
        Ok(user) => user,
        Err(Error::ElementNotFound) => return (StatusCode::NO_CONTENT).into_response(),
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
    };

    (StatusCode::OK, Json(User::from(user))).into_response()
}

pub async fn get_user_id(Path(user_id): Path<i64>) -> impl IntoResponse {
    let user = match database::get_user_by_id(&user_id).await {
        Ok(user) => user,
        Err(Error::ElementNotFound) => return (StatusCode::NO_CONTENT).into_response(),
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
    };

    (StatusCode::OK, Json(User::from(user))).into_response()
}
