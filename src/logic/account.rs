use crate::account::{BareAccount, EditAccount};
use crate::database;
use crate::logic::AuthContext;
use crate::logic::auth::logout;

use axum::extract::State;
use axum::response::IntoResponse;
use axum::Json;
use http::StatusCode;
use sqlx::SqlitePool;

/// Returns the current logged in account object
pub async fn get_account(auth: AuthContext) -> impl IntoResponse {
    (StatusCode::OK, Json(auth.current_user.unwrap())).into_response()
}

/// Creates a new account and returns the just created account object
pub async fn post_account(
    State(pool): State<&SqlitePool>,
    Json(payload): Json<BareAccount>,
) -> impl IntoResponse {
    // if username already exists, return with error
    if database::user::exists(pool, &payload.name).await {
        return (StatusCode::CONFLICT).into_response();
    }

    // create a new user
    let user = match database::account::insert(pool, &payload).await {
        Ok(user) => user,
        // todo check conflict error
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
    };

    (StatusCode::CREATED, Json(user)).into_response()
}

/// Edit the current logged in account
pub async fn edit_account(
    State(pool): State<&SqlitePool>,
    mut auth: AuthContext,
    Json(payload): Json<EditAccount>,
) -> impl IntoResponse {
    // todo ask for another password validation

    // edit the accounts information's from the database
    let updated_user =
        match database::account::update(pool, auth.current_user.unwrap().id, &payload).await {
            Ok(user) => user,
            // todo catch additional errors
            Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
        };

    (StatusCode::OK, Json(updated_user)).into_response()
}

/// Permanently delete the current logged in account
pub async fn delete_account(
    State(pool): State<&SqlitePool>,
    mut auth: AuthContext,
) -> impl IntoResponse {
    // todo ask for a password validation

    // delete the account from the database
    let deleted_user =
        match database::account::delete(pool, auth.clone().current_user.unwrap().id).await {
            Ok(user) => user,
            // todo catch additional errors
            Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
        };

    // after deletion log out the session
    logout(auth).await;

    (StatusCode::OK, Json(deleted_user)).into_response()
}

/// Change the password of the current logged in account
pub async fn edit_account_password(
    State(pool): State<&SqlitePool>,
    mut auth: AuthContext,
) -> impl IntoResponse {
    (StatusCode::NOT_IMPLEMENTED).into_response()
}
