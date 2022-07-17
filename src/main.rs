use axum::{
    routing::{get, patch, post},
    Extension, Router,
};
use dotenv::dotenv;
use futures::lock::Mutex;

use std::{net::SocketAddr, sync::Arc};

use crate::{
    api::cart_controller,
    application::{
        command::{
            add_to_cart_handler::AddToCartCommandHandler,
            create_cart_handler::CreateCartCommandHandler,
        },
        event::{
            added_to_cart_handler::AddedToCartEventHandler,
            created_cart_handler::CreatedCartEventHandler,
        },
    },
    services::{
        message_bus::{queue::MessageQueue, registry::HandlerRegistry, start_message_loop},
        persistence::event_store::EventStore,
    },
};

mod api;
mod application;
mod domain;
mod services;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let event_store = Arc::new(EventStore::new());

    let queue = Arc::new(Mutex::new(MessageQueue::new(event_store.clone())));
    let mut registry = HandlerRegistry::new(&event_store);

    // Commands
    registry.add(Box::new(AddToCartCommandHandler::new(queue.clone())));
    registry.add(Box::new(CreateCartCommandHandler::new(queue.clone())));

    // Events
    registry.add(Box::new(AddedToCartEventHandler::new()));
    registry.add(Box::new(CreatedCartEventHandler::new()));

    start_message_loop(queue.clone(), registry);

    let app = Router::new()
        .route("/cart", get(cart_controller::read))
        .route("/cart", post(cart_controller::create))
        .route("/cart", patch(cart_controller::update))
        .layer(Extension(queue));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
