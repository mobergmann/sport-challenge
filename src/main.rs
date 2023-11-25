mod activity;
mod hasher;
mod logic;
mod routes;
mod database;
mod account;

use axum::Router;

#[tokio::main]
async fn main() {
    // init database or exit program on error
    let pool = database::init().await.expect("Error while initializing the database.");

    let app = Router::new()
        .merge(routes::backend_router(&pool).await)
        .merge(routes::frontend_router().await);

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
