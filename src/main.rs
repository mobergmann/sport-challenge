mod activity;
mod hasher;
mod storage;
mod user;

use crate::activity::BareActivity;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{delete, get, post};
use axum::{Json, Router};
use axum_login::axum_sessions::{async_session::MemoryStore, SameSite, SessionLayer};
use axum_login::{AuthLayer, RequireAuthorizationLayer, SqliteStore};
use http::HeaderValue;
use rand::Rng;
use sqlx::sqlite::SqlitePoolOptions;
use tower_http::cors::{Any, CorsLayer};
use crate::storage::{Error, DB_URI};
use crate::user::{BareUser, User};

type AuthContext = axum_login::extractors::AuthContext<i64, User, SqliteStore<User>>;

#[tokio::main]
async fn main() {
    let secret = rand::thread_rng().gen::<[u8; 64]>();

    let session_store = MemoryStore::new();
    let session_layer = SessionLayer::new(session_store, &secret).with_secure(false).with_same_site_policy(SameSite::None);

    let pool = SqlitePoolOptions::new().connect(DB_URI).await.unwrap();

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
        .route("/v1/activities/edit", post(edit_activity))
        .route("/v1/activities/:id", delete(delete_activity))
        // routes above are protected
        .route_layer(RequireAuthorizationLayer::<i64, User>::login())
        // authentication routes
        .route("/v1/auth/sign_up", post(sign_up))
        .route("/v1/auth/sign_in", post(sign_in))
        .route("/v1/auth/sign_out", get(sign_out))
        .layer(CorsLayer::very_permissive())
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
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "an unknown server error occurred. please contact the administrator.",
        )
            .into_response(),
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

async fn get_activity(mut auth: AuthContext, Path(activity_id): Path<i64>) -> impl IntoResponse {
    let activity = match storage::get_activity(activity_id).await {
        Ok(activity) => activity,
        Err(Error::ElementNotFound) => return (StatusCode::NOT_FOUND).into_response(),
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "an unknown server error occurred. please contact the administrator.",
            )
                .into_response()
        }
    };
    (StatusCode::OK, Json(activity)).into_response()
}

async fn get_activities(mut auth: AuthContext) -> impl IntoResponse {
    let activities = match storage::get_all_activities().await {
        Ok(activities) => activities,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "an unknown server error occurred. please contact the administrator.",
            )
                .into_response()
        }
    };
    (StatusCode::OK, Json(activities)).into_response()
}

async fn new_activity(
    mut auth: AuthContext,
    Json(payload): Json<BareActivity>,
) -> impl IntoResponse {
    let activity = match storage::new_activity(&payload, &auth.current_user.unwrap()).await {
        Ok(activity) => activity,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "an unknown server error occurred. please contact the administrator.",
            )
                .into_response()
        }
    };
    (StatusCode::OK, Json(activity)).into_response()
}

async fn edit_activity(
    mut auth: AuthContext,
    Json(payload): Json<BareActivity>,
) -> impl IntoResponse {
    (
        StatusCode::NOT_IMPLEMENTED,
        "this request is currently not implemented",
    )
        .into_response()
}

async fn delete_activity(mut auth: AuthContext, Path(activity_id): Path<i64>) -> impl IntoResponse {
    let activity = match storage::get_activity(activity_id).await {
        Ok(activity) => activity,
        Err(Error::ElementNotFound) => return (StatusCode::NOT_FOUND).into_response(),
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "an unknown server error occurred. please contact the administrator.",
            )
                .into_response()
        }
    };

    // only the activity author is allowed to delete its activities
    if activity.author_id != auth.current_user.unwrap().id {
        return (
            StatusCode::UNAUTHORIZED,
            "only the activity owner can delete an activity",
        )
            .into_response();
    }

    let activity = match storage::delete_activity(activity_id).await {
        Ok(activity) => activity,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "an unknown server error occurred. please contact the administrator.",
            )
                .into_response()
        }
    };
    (StatusCode::OK, Json(activity)).into_response()
}

async fn get_account() {}

async fn edit_account() {}

async fn delete_account() {}
