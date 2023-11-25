use crate::account::Account;
use crate::database::DB_URI;
use crate::logic::account::{
    delete_account, edit_account, edit_account_password, get_account, post_account,
};
use crate::logic::activities::{
    delete_activity, edit_activity, get_activities_from_to, get_activity, post_activity,
};
use crate::logic::auth::{login, logout};
use crate::logic::users::{get_user, get_user_id};

use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{delete, get, post, put};
use axum::Router;
use axum_login::axum_sessions::async_session::MemoryStore;
use axum_login::axum_sessions::{SameSite, SessionLayer};
use axum_login::{AuthLayer, RequireAuthorizationLayer, SqliteStore};
use rand::Rng;
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::SqlitePool;
use tower_http::services::ServeDir;

#[allow(clippy::unused_async)]
async fn handle_error() -> impl IntoResponse {
    (StatusCode::INTERNAL_SERVER_ERROR).into_response()
}

pub async fn frontend_router() -> Router {
    Router::new().nest_service("/", ServeDir::new("public"))
}

pub async fn backend_router(pool: &SqlitePool) -> Router {
    let secret = rand::thread_rng().gen::<[u8; 64]>(); // todo use secret from environment variable

    let session_store = MemoryStore::new();
    let session_layer = SessionLayer::new(session_store, &secret)
        .with_secure(false)
        .with_same_site_policy(SameSite::Lax);
    let user_store = SqliteStore::<Account>::new(pool.clone());
    let auth_layer = AuthLayer::new(user_store, &secret);

    let auth_router = Router::new()
        .route("/v1/auth/login", post(login))
        .route("/v1/auth/logout", put(logout))
        .layer(auth_layer.clone())
        .layer(session_layer.clone());

    let account_router = Router::new()
        .route("/v1/account", get(get_account))
        .route("/v1/account", post(post_account))
        .route("/v1/account", put(edit_account))
        .route("/v1/account", delete(delete_account))
        .route("/v1/account/password", put(edit_account_password))
        .route_layer(RequireAuthorizationLayer::<i64, Account>::login())
        .layer(auth_layer.clone())
        .layer(session_layer.clone());

    let users_router = Router::new()
        .route("/v1/users/id/:id", get(get_user_id))
        .route("/v1/users/:username", get(get_user))
        .route_layer(RequireAuthorizationLayer::<i64, Account>::login())
        .layer(auth_layer.clone())
        .layer(session_layer.clone());

    let activities_router = Router::new()
        .route("/v1/activities", post(post_activity))
        .route("/v1/activities/:id", get(get_activity))
        .route("/v1/activities/:id", put(edit_activity))
        .route("/v1/activities/:id", delete(delete_activity))
        .route("/v1/activities/:from/:to", get(get_activities_from_to))
        .route_layer(RequireAuthorizationLayer::<i64, Account>::login())
        .layer(auth_layer.clone())
        .layer(session_layer.clone());

    Router::new()
        .merge(auth_router)
        .merge(account_router)
        .merge(users_router)
        .merge(activities_router)
        .with_state(pool)
}
