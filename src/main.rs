use axum::{
    response::IntoResponse,
    routing::{get, post},
    Extension, Router,
};
use dotenv::dotenv;
use uuid::Uuid;

use std::any::TypeId;
use std::net::SocketAddr;

use crate::api::cart_controller;
use crate::application::event::{
    added_to_cart_event::AddedToCartEvent, added_to_cart_handler::AddedToCartEventHandler,
    created_cart_event::CreatedCartEvent, created_cart_handler::CreatedCartEventHandler,
};
use crate::services::message_bus::bus::MessageBus;

mod api;
mod application;
mod domain;
mod services;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let mut bus = MessageBus::new();

    let added_to_cart_handler = AddedToCartEventHandler::new();
    let created_card_handler = CreatedCartEventHandler::new();

    bus.register_handler(&added_to_cart_handler);
    bus.register_handler(&created_card_handler);

    let test_message = AddedToCartEvent::new(Uuid::new_v4(), Uuid::new_v4(), Uuid::new_v4(), 0);
    bus.send(TypeId::of::<AddedToCartEvent>(), &test_message);

    let app = Router::new()
        .route("/", get(test))
        .route("/cart", post(cart_controller::insert));
    // .layer(Extension(bus));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

pub async fn test() -> impl IntoResponse {
    "It works!"
}
