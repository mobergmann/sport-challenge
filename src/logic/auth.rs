use crate::database;
use crate::hasher;
use crate::account::{BareAccount, Account};

use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use axum_login::SqliteStore;

type AuthContext = axum_login::extractors::AuthContext<i64, Account, SqliteStore<Account>>;

pub async fn login(mut auth: AuthContext, Json(payload): Json<BareAccount>) -> impl IntoResponse {
    let user = match database::get_user(&payload.name).await {
        Ok(user) => user,
        Err(_) => return (StatusCode::NOT_FOUND, "name does not exist").into_response(),
    };

    if !hasher::verify(&user.password_hash, &payload.password) {
        return (StatusCode::UNAUTHORIZED, "password doesn't match").into_response();
    }

    auth.login(&user).await.unwrap();
    (StatusCode::OK, Json(user)).into_response()
}

pub async fn logout(mut auth: AuthContext) -> impl IntoResponse {
    auth.logout().await;
    (StatusCode::OK).into_response()
}
