mod hasher;
mod storage;
mod user;

use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{delete, get, post};
use axum::{Json, Router};
use axum_login::axum_sessions::{async_session::MemoryStore, SessionLayer};
use axum_login::{AuthLayer, RequireAuthorizationLayer, SqliteStore};
use rand::Rng;
use sqlx::sqlite::SqlitePoolOptions;

use crate::storage::USER_DB;
use crate::user::{BareUser, User};

type AuthContext = axum_login::extractors::AuthContext<i64, User, SqliteStore<User>>;

#[tokio::main]
async fn main() {
    let secret = rand::thread_rng().gen::<[u8; 64]>();

    let session_store = MemoryStore::new();
    let session_layer = SessionLayer::new(session_store, &secret).with_secure(false);

    let pool = SqlitePoolOptions::new().connect(USER_DB).await.unwrap();

    let user_store = SqliteStore::<User>::new(pool);
    let auth_layer = AuthLayer::new(user_store, &secret);

    let app = Router::new()
        // account management routes
        .route("/v1/account", get(get_account))
        .route("/v1/account/edit", post(edit_account))
        .route("/v1/account", delete(delete_account))
        // activity routes
        .route("/v1/activities/:id", get(get_activity))
        .route("/v1/activities", get(get_activities))
        .route("/v1/activities", post(new_activity))
        .route("/v1/activities", delete(delete_activity))
        // routes above are protected
        .route_layer(RequireAuthorizationLayer::<i64, User>::login())
        // authentication routes
        .route("/v1/auth/sign_up", post(sign_up))
        .route("/v1/auth/sign_in", post(sign_in))
        .route("/v1/auth/sign_out", get(sign_out))
        .layer(auth_layer)
        .layer(session_layer);

    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn sign_up(Json(payload): Json<BareUser>) -> impl IntoResponse {
    // if username already exists, return with error
    if storage::user_exists(&payload.name).await {
        return (StatusCode::CONFLICT, "User with the name already exists").into_response();
    }

    // create a new user
    match storage::insert_new_user(&payload).await {
        Ok(user) => (StatusCode::CREATED, Json(user)).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR,
                   "an unknown server error occurred. please contact the administrator.").into_response(),
    }
}

async fn sign_in(mut auth: AuthContext, Json(payload): Json<BareUser>) -> impl IntoResponse {
    let user = match storage::get_user(&payload.name).await {
        Ok(user) => user,
        Err(_) => return (StatusCode::NOT_FOUND, "name does not exist").into_response(),
    };

    if !hasher::verify(&user.password_hash, &payload.password) {
        return (StatusCode::UNAUTHORIZED, "password doesn't match").into_response();
    }

    auth.login(&user).await.unwrap();
    (StatusCode::OK, Json(user)).into_response()
}

async fn sign_out(mut auth: AuthContext) -> impl IntoResponse {
    auth.logout().await;
    (StatusCode::OK).into_response()
}

async fn get_activity(Path(activity_id): Path<u64>) {}

async fn get_activities() {}

async fn new_activity() {}

async fn delete_activity() {}

async fn get_account() {}

async fn edit_account() {}

async fn delete_account() {}
