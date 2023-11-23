use crate::activity::{BareActivity, StringBareActivity};
use crate::database;
use crate::database::Error;
use crate::logic::AuthContext;

use axum::extract::Path;
use axum::response::IntoResponse;
use axum::Json;
use chrono::{DateTime, Utc};
use http::StatusCode;

/// Returns a single `Activity` by id
pub async fn get_activity(Path(activity_id): Path<i64>) -> impl IntoResponse {
    let activity = match database::get_activity(activity_id).await {
        Ok(activity) => activity,
        Err(Error::ElementNotFound) => return (StatusCode::NOT_FOUND).into_response(),
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
    };

    (StatusCode::OK, Json(activity)).into_response()
}

/// Returns a list of `Activity` which were started in an time intervall of [:from, :to]
pub async fn get_activities_from_to(Path((from, to)): Path<(String, String)>) -> impl IntoResponse {
    // parse the :from parameter as a RFC-3339 DateTime String
    // otherwise return an error
    let from = match DateTime::parse_from_rfc3339(&from) {
        Ok(time) => time.with_timezone(&Utc),
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(":from url-parameter is not a valid rfc3339 format"),
            )
                .into_response();
        } // todo return error code which is same as axum internal error codes
    };

    // parse the :to parameter as a RFC-3339 DateTime String
    // otherwise return an error
    let to = match DateTime::parse_from_rfc3339(&to) {
        Ok(time) => time.with_timezone(&Utc),
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(":to url-parameter is not a valid rfc3339 format"),
            )
                .into_response();
        } // todo return error code which is same as axum internal error codes
    };

    if to < from {
        return (
            StatusCode::BAD_REQUEST,
            Json("the :to time must be later than the :from time"),
        )
            .into_response();
        // todo return error code which is same as axum internal error codes
    }

    let activities = match database::get_activities(&from, &to).await {
        Ok(activities) => activities,
        // todo catch additional errors
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
    };

    (StatusCode::OK, Json(activities)).into_response()
}

/// Creates a new `Activity` object
pub async fn post_activity(
    mut auth: AuthContext,
    Json(payload): Json<StringBareActivity>,
) -> impl IntoResponse {
    let start_time = match DateTime::parse_from_rfc3339(&payload.start_time) {
        Ok(time) => time.with_timezone(&Utc),
        // todo catch additional errors
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                Json("start_time is not a valid rfc3339 format"),
            )
                .into_response();
        }
    };

    let end_time = match DateTime::parse_from_rfc3339(&payload.end_time) {
        Ok(time) => time.with_timezone(&Utc),
        // todo catch additional errors
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                Json("end_time is not a valid rfc3339 format"),
            )
                .into_response();
        }
    };

    if end_time < start_time {
        return (
            StatusCode::BAD_REQUEST,
            Json("the end_time time must be later than the start_time"),
        )
            .into_response();
    }

    let author_id = auth.current_user.unwrap().id;
    let new_activity = BareActivity {
        amount: payload.amount,
        activity_type: payload.activity_type,
        start_time: start_time,
        end_time: end_time,
    };

    let activity = match database::new_activity(&new_activity, &author_id).await {
        Ok(activity) => activity,
        // todo catch additional errors
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
    };

    (StatusCode::OK, Json(activity)).into_response()
}

/// Edits the information of an `Activity` object
pub async fn edit_activity(
    mut auth: AuthContext,
    Path(activity_id): Path<(i64)>,
    Json(payload): Json<StringBareActivity>,
) -> impl IntoResponse {
    // get the referenced activity from the database
    let activity = match database::get_activity(activity_id).await {
        Ok(activity) => activity,
        Err(Error::ElementNotFound) => return (StatusCode::NOT_FOUND).into_response(),
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
    };

    let author_id = auth.current_user.unwrap().id;

    // only the activity author is allowed to delete its activities
    if activity.author_id != author_id {
        return (StatusCode::UNAUTHORIZED).into_response();
    }

    // update the activity in the database
    let activity = match database::edit_activity(payload, author_id).await {
        Ok(activity) => activity,
        // todo catch additional errors
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
    };

    (StatusCode::OK, Json(activity)).into_response()
}

/// Deletes an `Activity` object
pub async fn delete_activity(
    mut auth: AuthContext,
    Path(activity_id): Path<(i64)>,
) -> impl IntoResponse {
    // get the referenced activity from the database
    let activity = match database::get_activity(activity_id).await {
        Ok(activity) => activity,
        Err(Error::ElementNotFound) => return (StatusCode::NOT_FOUND).into_response(),
        // todo catch additional errors
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
    };

    let author_id = auth.current_user.unwrap().id;

    // only the activity author is allowed to delete its activities
    if activity.author_id != author_id {
        return (StatusCode::UNAUTHORIZED).into_response();
    }

    // delete the activity from the database
    let activity = match database::delete_activity(activity_id).await {
        Ok(activity) => activity,
        // todo catch additional errors
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
    };

    (StatusCode::OK, Json(activity)).into_response()
}
