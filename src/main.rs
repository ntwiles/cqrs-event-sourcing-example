use axum::{
    routing::{get, post},
    Extension, Router,
};
use dotenv::dotenv;
use futures::lock::Mutex;

use std::{net::SocketAddr, sync::Arc};

use crate::{
    api::{cart_controller, user_controller},
    application::{
        command::cart_add_item_handler::CartAddItemHandler,
        event::{
            cart_item_added_handler::CartItemAddedHandler,
            user_cart_created_handler::UserCartCreatedHandler,
        },
        query::{cart::CartStore, user::UserStore},
    },
    infrastructure::{
        message_bus::{queue::MessageQueue, registry::HandlerRegistry, start_message_loop},
        persistence::events::EventService,
    },
};

mod api;
mod application;
mod domain;
mod infrastructure;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let events_service = Arc::new(EventService::new().await);

    let cart_store = Arc::new(CartStore::new(events_service.clone()));
    let user_store = Arc::new(UserStore::new(events_service.clone()));

    let bus = Arc::new(Mutex::new(MessageQueue::new(events_service.clone())));
    let mut registry = HandlerRegistry::new(&events_service);

    // commands
    registry.add(Box::new(CartAddItemHandler::new(bus.clone())));

    // events
    registry.add(Box::new(CartItemAddedHandler::new()));
    registry.add(Box::new(UserCartCreatedHandler::new()));

    start_message_loop(bus.clone(), registry);

    let app = Router::new()
        .route("/cart", get(cart_controller::read))
        .route("/cart", post(cart_controller::update))
        .route("/user/:user_id", get(user_controller::read))
        .layer(Extension(bus))
        .layer(Extension(cart_store))
        .layer(Extension(user_store));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
