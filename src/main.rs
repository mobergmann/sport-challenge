mod activity;
mod hasher;
mod logic;
mod services;
mod storage;
mod user;

use axum::Router;
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .merge(services::backend_rouer().await)
        .merge(services::frontend_router().await)
        .layer(CorsLayer::very_permissive());

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
