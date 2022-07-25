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
        query::cart_store::CartStore,
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

    let queue = Arc::new(Mutex::new(MessageQueue::new(events_service.clone())));
    let mut registry = HandlerRegistry::new(&events_service);

    // commands
    registry.add(Box::new(AddToCartCommandHandler::new(queue.clone())));
    registry.add(Box::new(CreateCartCommandHandler::new(queue.clone())));

    // events
    registry.add(Box::new(AddedToCartEventHandler::new()));
    registry.add(Box::new(CreatedCartEventHandler::new()));

    start_message_loop(queue.clone(), registry);

    let app = Router::new()
        .route("/cart", get(cart_controller::read))
        .route("/cart", post(cart_controller::create))
        .route("/cart", patch(cart_controller::update))
        .layer(Extension(queue))
        .layer(Extension(cart_store));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
