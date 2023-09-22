use axum::extract::Path;
use axum::Router;
use axum::routing::{delete, get, post};

#[tokio::main]
async fn main() {
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

        // authentication routes
        .route("/v1/auth/sign_up", post(sign_up))
        .route("/v1/auth/sign_in", post(sign_in))
        .route("/v1/auth/sign_out", get(sign_out));

    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn sign_up() {
}

async fn sign_in() {
}

async fn sign_out() {
}


async fn get_activity(Path(activity_id): Path<u64>) {
}

async fn get_activities() {
}

async fn new_activity() {
}

async fn delete_activity() {
}


async fn get_account() {
}

async fn edit_account() {
}

async fn delete_account() {
}
