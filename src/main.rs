mod activity;
mod hasher;
mod logic;
mod services;
mod storage;
mod user;

use axum::Router;
use rand::Rng;
use std::env;

#[tokio::main]
async fn main() {
    // init database or exit program on error
    match storage::init().await {
        Ok(_) => {}
        Err(_) => panic!("Error while initializing the database."),
    };

    // get session secret from environment variable
    // default to random session secret
    let secret:[u8; 64] = match env::var("SESSION_SECRET") {
        Ok(var) => {
            let mut array_tmp = [0u8; 64];
            array_tmp[..var.len()].copy_from_slice(var.as_bytes());
            array_tmp
        },
        Err(_) => rand::thread_rng().gen::<[u8; 64]>(),
    };

    let app = Router::new()
        .merge(services::backend_router(secret).await)
        .merge(services::frontend_router().await);

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
