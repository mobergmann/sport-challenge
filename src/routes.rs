use crate::logic::*;
use crate::user::User;

use crate::database::DB_URI;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{delete, get, put, post};
use axum::Router;
use axum_login::axum_sessions::async_session::MemoryStore;
use axum_login::axum_sessions::{SameSite, SessionLayer};
use axum_login::{AuthLayer, RequireAuthorizationLayer, SqliteStore};
use rand::Rng;
use sqlx::sqlite::SqlitePoolOptions;
use tower_http::services::ServeDir;

#[allow(clippy::unused_async)]
async fn handle_error() -> impl IntoResponse {
    (StatusCode::INTERNAL_SERVER_ERROR).into_response()
}

pub async fn frontend_router() -> Router {
    Router::new().nest_service("/", ServeDir::new("public"))
}

pub async fn backend_router() -> Router {
    let secret = rand::thread_rng().gen::<[u8; 64]>(); // todo use secret from environment variable

    let session_store = MemoryStore::new();
    let session_layer = SessionLayer::new(session_store, &secret)
        .with_secure(false)
        .with_same_site_policy(SameSite::Lax);
    let pool = SqlitePoolOptions::new().connect(DB_URI).await.unwrap();
    let user_store = SqliteStore::<User>::new(pool);
    let auth_layer = AuthLayer::new(user_store, &secret);

    let auth_router = Router::new()
        .route("/v1/auth/ping", get(ping))
        .route_layer(RequireAuthorizationLayer::<i64, User>::login())
        .route("/v1/auth/login", post(login))
        .route("/v1/auth/logout", put(logout))
        .layer(&auth_layer)
        .layer(&session_layer);

    let account_router = Router::new()
        .route("/v1/account", get(get_account))
        .route("/v1/account", post(new_account))
        .route("/v1/account", put(edit_account))
        .route("/v1/account", delete(delete_account))
        .route("/v1/account/password", put(change_account_password))
        .route_layer(RequireAuthorizationLayer::<i64, User>::login())
        .layer(&auth_layer)
        .layer(&session_layer);

    let users_router = Router::new()
        .route("/v1/users/id/:id", get(get_user_by_id))
        .route("/v1/users/:username", get(get_user))
        .route_layer(RequireAuthorizationLayer::<i64, User>::login())
        .layer(&auth_layer)
        .layer(&session_layer);

    let activities_router = Router::new()
        .route("/v1/activities", post(new_activity))
        .route("/v1/activities/:id", get(get_activity))
        .route("/v1/activities/:id", put(edit_activity))
        .route("/v1/activities/:id", delete(delete_activity))
        .route("/v1/activities/:from/:to", get(get_activities_from_to))
        .route_layer(RequireAuthorizationLayer::<i64, User>::login())
        .layer(&auth_layer)
        .layer(&session_layer);

    Router::new()
        .merge(auth_router)
        .merge(account_router)
        .merge(users_router)
        .merge(activities_router)
}
