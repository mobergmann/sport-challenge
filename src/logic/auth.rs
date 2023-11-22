use crate::database;
use crate::hasher;
use crate::user::{BareUser, User};

use axum_login::SqliteStore;
use axum::response::IntoResponse;
use axum::http::StatusCode;
use axum::Json;

type AuthContext = axum_login::extractors::AuthContext<i64, User, SqliteStore<User>>;

pub async fn sign_in(mut auth: AuthContext, Json(payload): Json<BareUser>) -> impl IntoResponse
{
    let user = match database::get_user(&payload.name).await
    {
        Ok(user) => user,
        Err(_) => return (StatusCode::NOT_FOUND, "name does not exist").into_response(),
    };

    if !hasher::verify(&user.password_hash, &payload.password)
    {
        return (StatusCode::UNAUTHORIZED, "password doesn't match").into_response();
    }

    auth.login(&user).await.unwrap();
    (StatusCode::OK, Json(user)).into_response()
}

pub async fn sign_out(mut auth: AuthContext) -> impl IntoResponse
{
    auth.logout().await;
    (StatusCode::OK).into_response()
}

pub async fn ping() -> impl IntoResponse
{
    (StatusCode::OK).into_response()
}
