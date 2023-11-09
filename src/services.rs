use crate::logic::*;
use crate::user::User;

use crate::storage::DB_URI;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{delete, get, post};
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
    let secret = rand::thread_rng().gen::<[u8; 64]>();

    let session_store = MemoryStore::new();
    let session_layer = SessionLayer::new(session_store, &secret)
        .with_secure(false)
        .with_same_site_policy(SameSite::Lax);

    let pool = SqlitePoolOptions::new().connect(DB_URI).await.unwrap();

    let user_store = SqliteStore::<User>::new(pool);
    let auth_layer = AuthLayer::new(user_store, &secret);

    Router::new()
        // account management routes
        .route("/v1/account", get(get_account))
        .route("/v1/account/edit", post(edit_account))
        .route("/v1/account", delete(delete_account))
        // user
        .route("/v1/user/id/:id", get(get_user_by_id))
        .route("/v1/user/:username", get(get_user))
        // activity routes
        .route("/v1/activities/:id", get(get_activity))
        .route("/v1/activities/:from/:to", get(get_activities_from_to))
        .route("/v1/activities", post(new_activity))
        .route("/v1/activities/edit", post(edit_activity))
        .route("/v1/activities/:id", delete(delete_activity))
        // routes above are protected
        .route_layer(RequireAuthorizationLayer::<i64, User>::login())
        // authentication routes
        .route("/v1/auth/sign_up", post(sign_up))
        .route("/v1/auth/sign_in", post(sign_in))
        .route("/v1/auth/sign_out", get(sign_out))
        .layer(auth_layer)
        .layer(session_layer)
}
