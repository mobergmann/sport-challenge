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
    match database::init().await {
        Ok(_) => {},
        Err(_) => panic!("Error while initializing the database."),
    };

    let app = Router::new()
        .merge(routes::backend_router().await)
        .merge(routes::frontend_router().await);

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
