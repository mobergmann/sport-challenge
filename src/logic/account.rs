use crate::account::{BareAccount, EditAccount};
use crate::{database, hasher};
use crate::logic::AuthContext;
use crate::logic::auth::logout;
use crate::database::Error;

use axum::extract::State;
use axum::response::IntoResponse;
use axum::Json;
use http::StatusCode;
use serde::Deserialize;
use sqlx::SqlitePool;

#[derive(Deserialize)]
pub struct PasswordValidation {
    pub current_password: String,
}

#[derive(Deserialize)]
pub struct EditPassword {
    pub current_password: String,
    pub new_password: String,
}

/// Returns the current logged in account object
pub async fn get_account(auth: AuthContext) -> impl IntoResponse {
    (StatusCode::OK, Json(auth.current_user.unwrap())).into_response()
}

/// Creates a new account and returns the just created account object
pub async fn post_account(
    State(pool): State<SqlitePool>,
    Json(payload): Json<BareAccount>,
) -> impl IntoResponse {
    // if username already exists, return with error
    if database::user::exists(pool.clone(), &payload.username).await {
        return (StatusCode::CONFLICT).into_response();
    }

    // create a new user
    let user = match database::account::insert(pool, &payload).await {
        Ok(user) => user,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
    };

    (StatusCode::CREATED, Json(user)).into_response()
}

/// Edit the current logged in account
pub async fn edit_account(
    State(pool): State<SqlitePool>,
    auth: AuthContext,
    Json(payload): Json<EditAccount>,
) -> impl IntoResponse {
    // if username already exists, return with error
    if database::user::exists(pool.clone(), &payload.username).await {
        return (StatusCode::CONFLICT).into_response();
    }

    // edit the accounts information's from the database
    let updated_user =
        match database::account::update(pool, auth.current_user.unwrap().id, &payload).await {
            Ok(user) => user,
            Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
        };

    (StatusCode::OK, Json(updated_user)).into_response()
}

/// Permanently delete the current logged in account
pub async fn delete_account(
    State(pool): State<SqlitePool>,
    auth: AuthContext,
    Json(payload): Json<PasswordValidation>,
) -> impl IntoResponse {
    let account = match database::account::get_id(pool.clone(), auth.clone().current_user.unwrap().id).await {
        Ok(user) => user,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
    };

    if !hasher::verify(&account.password_hash, &payload.current_password) {
        return (StatusCode::UNAUTHORIZED).into_response();
    }

    // delete the account from the database
    let account =
        match database::account::delete(pool, auth.clone().current_user.unwrap().id).await {
            Ok(user) => user,
            Err(Error::SQLX(e)) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
            Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
        };

    // after deletion log out the session
    logout(auth).await;

    (StatusCode::OK, Json(account)).into_response()
}

/// Change the password of the current logged in account
pub async fn edit_account_password(
    State(pool): State<SqlitePool>,
    auth: AuthContext,
    Json(payload): Json<EditPassword>,
) -> impl IntoResponse {
    let account = match database::account::get_id(pool.clone(), auth.clone().current_user.unwrap().id).await {
        Ok(user) => user,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
    };

    if !hasher::verify(&account.password_hash, &payload.current_password) {
        return (StatusCode::UNAUTHORIZED).into_response();
    }

    // delete the account from the database
    let account =
        match database::account::update_password(pool, auth.clone().current_user.unwrap().id, payload.new_password).await {
            Ok(user) => user,
            Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
        };

    // after deletion log out the session
    logout(auth).await;

    (StatusCode::OK, Json(account)).into_response()
}
