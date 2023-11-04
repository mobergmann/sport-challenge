mod activity;
mod hasher;
mod logic;
mod services;
mod storage;
mod user;

use axum::Router;

#[tokio::main]
async fn main() {
    // init database or exit program on error
    match storage::init().await {
        Ok(_) => {},
        Err(_) => panic!("Error while initializing the database."),
    };

    let app = Router::new()
        .merge(services::backend_router().await)
        .merge(services::frontend_router().await);

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
