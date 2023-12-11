use crate::account::{Account, BareAccount};
use crate::database;
use crate::hasher;

use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use axum_login::SqliteStore;
use sqlx::SqlitePool;
use crate::database::Error;

type AuthContext = axum_login::extractors::AuthContext<i64, Account, SqliteStore<Account>>;

pub async fn login(
    State(pool): State<SqlitePool>,
    mut auth: AuthContext,
    Json(payload): Json<BareAccount>,
) -> impl IntoResponse {
    let user = match database::account::get(pool, &payload.username).await {
        Ok(user) => user,
        Err(_) => return (StatusCode::NOT_FOUND, "user with the name doesn't exist").into_response(),
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
