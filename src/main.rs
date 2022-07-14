use axum::{
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use dotenv::dotenv;

use std::net::SocketAddr;

use crate::api::cart_controller;

mod api;
mod application;
mod domain;
mod services;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let app = Router::new()
        .route("/", get(test))
        .route("/cart", post(cart_controller::insert));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

pub async fn test() -> impl IntoResponse {
    "It works!"
}
