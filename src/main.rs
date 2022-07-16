use axum::{
    response::IntoResponse,
    routing::{get, post},
    Extension, Router,
};
use dotenv::dotenv;
use uuid::Uuid;

use std::net::SocketAddr;
use std::sync::Arc;

use crate::api::cart_controller;
use crate::application::event::{
    added_to_cart_event::AddedToCartEvent, added_to_cart_handler::AddedToCartEventHandler,
    created_cart_handler::CreatedCartEventHandler,
};
use crate::services::{message_bus::bus::MessageBus, persistence::event_store::EventStore};

mod api;
mod application;
mod domain;
mod services;

struct Foo {}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let event_store = EventStore::new();

    let mut bus = MessageBus::new(&event_store);

    let added_to_cart_handler = AddedToCartEventHandler::new();
    let created_card_handler = CreatedCartEventHandler::new();

    bus.register_handler(Box::new(added_to_cart_handler));
    bus.register_handler(Box::new(created_card_handler));

    let test_message = AddedToCartEvent::new(Uuid::new_v4(), Uuid::new_v4(), Uuid::new_v4(), 0);
    bus.send(Box::new(test_message)).await;

    let shared_bus = Arc::new(bus);

    let app = Router::new()
        .layer(Extension(shared_bus))
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
