mod activity;
mod hasher;
mod logic;
mod services;
mod storage;
mod user;

use axum::Router;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .merge(services::backend_rouer().await)
        .merge(services::frontend_router().await);

    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
