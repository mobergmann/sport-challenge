use crate::activity::{BareActivity, StringBareActivity};
use crate::storage::Error;
use crate::user::{BareUser, PublicUser, User};
use crate::{hasher, storage};
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use axum_login::SqliteStore;
use chrono::DateTime;

type AuthContext = axum_login::extractors::AuthContext<i64, User, SqliteStore<User>>;

pub async fn sign_up(Json(payload): Json<BareUser>) -> impl IntoResponse {
    // if username already exists, return with error
    if storage::user_exists(&payload.name).await {
        return (StatusCode::CONFLICT).into_response();
    }

    // create a new user
    match storage::insert_new_user(&payload).await {
        Ok(user) => (StatusCode::CREATED, Json(user)).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
    }
}

pub async fn sign_in(mut auth: AuthContext, Json(payload): Json<BareUser>) -> impl IntoResponse {
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

pub async fn sign_out(mut auth: AuthContext) -> impl IntoResponse {
    auth.logout().await;
    (StatusCode::OK).into_response()
}

pub async fn get_activity(
    mut auth: AuthContext,
    Path(activity_id): Path<i64>,
) -> impl IntoResponse {
    let activity = match storage::get_activity(activity_id).await {
        Ok(activity) => activity,
        Err(Error::ElementNotFound) => return (StatusCode::NOT_FOUND).into_response(),
        Err(_) => {
            return (StatusCode::INTERNAL_SERVER_ERROR).into_response()
        }
    };
    (StatusCode::OK, Json(activity)).into_response()
}

pub async fn get_activities(mut auth: AuthContext) -> impl IntoResponse {
    let activities = match storage::get_all_activities().await {
        Ok(activities) => activities,
        Err(_) => {
            return (StatusCode::INTERNAL_SERVER_ERROR).into_response()
        }
    };
    (StatusCode::OK, Json(activities)).into_response()
}

pub async fn new_activity(
    mut auth: AuthContext,
    Json(payload): Json<StringBareActivity>,
) -> impl IntoResponse {
    let start_time = match DateTime::parse_from_rfc3339(payload.start_time.as_str()) {
        Ok(time) => time,
        Err(_) => {
            return (StatusCode::BAD_REQUEST, Json("not a valid rfc3339 date string string")).into_response();
        }
    };
    let end_time = match DateTime::parse_from_rfc3339(payload.end_time.as_str()) {
        Ok(time) => time,
        Err(_) => {
            return (StatusCode::BAD_REQUEST, Json("not a valid rfc3339 date string string")).into_response();
        }
    };
    if end_time < start_time {
        return (StatusCode::BAD_REQUEST, Json("the end time point of the activity cannot be later than the beginning time point of the activity")).into_response();
    }

    let converted_activity = BareActivity {
        amount: payload.amount,
        activity_type: payload.activity_type,
        start_time: DateTime::from(start_time),
        end_time: DateTime::from(end_time),
    };

    let activity = match storage::new_activity(&converted_activity, &auth.current_user.unwrap()).await {
        Ok(activity) => activity,
        Err(_) => {
            return (StatusCode::INTERNAL_SERVER_ERROR).into_response()
        }
    };
    (StatusCode::OK, Json(activity)).into_response()
}

pub async fn edit_activity(
    mut auth: AuthContext,
    Json(payload): Json<StringBareActivity>,
) -> impl IntoResponse {
    (StatusCode::NOT_IMPLEMENTED).into_response()
}

pub async fn delete_activity(
    mut auth: AuthContext,
    Path(activity_id): Path<i64>,
) -> impl IntoResponse {
    let activity = match storage::get_activity(activity_id).await {
        Ok(activity) => activity,
        Err(Error::ElementNotFound) => return (StatusCode::NOT_FOUND).into_response(),
        Err(_) => {
            return (StatusCode::INTERNAL_SERVER_ERROR).into_response()
        }
    };

    // only the activity author is allowed to delete its activities
    if activity.author_id != auth.current_user.unwrap().id {
        return (StatusCode::UNAUTHORIZED).into_response();
    }

    let activity = match storage::delete_activity(activity_id).await {
        Ok(activity) => activity,
        Err(_) => {
            return (StatusCode::INTERNAL_SERVER_ERROR).into_response()
        }
    };
    (StatusCode::OK, Json(activity)).into_response()
}

pub async fn get_user(mut auth: AuthContext, Path(username): Path<String>) -> impl IntoResponse
{
    let user = match storage::get_user(&username).await {
        Ok(user) => user,
        Err(Error::ElementNotFound) => return (StatusCode::NO_CONTENT).into_response(),
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
    };

    (StatusCode::OK, Json(PublicUser::from(user))).into_response()
}

pub async fn get_user_by_id(mut auth: AuthContext, Path(user_id): Path<i64>) -> impl IntoResponse
{
    let user = match storage::get_user_by_id(&user_id).await {
        Ok(user) => user,
        Err(Error::ElementNotFound) => return (StatusCode::NO_CONTENT).into_response(),
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
    };

    (StatusCode::OK, Json(PublicUser::from(user))).into_response()
}

pub async fn get_account() {}

pub async fn edit_account() {}

pub async fn delete_account() {}
