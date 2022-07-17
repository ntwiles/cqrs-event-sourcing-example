use axum::{
    routing::{get, patch, post},
    Extension, Router,
};
use dotenv::dotenv;
use futures::lock::Mutex;
use maplit::hashmap;

use std::{any::TypeId, net::SocketAddr, sync::Arc};

use crate::{
    api::cart_controller,
    application::{
        command::{
            add_to_cart_handler::AddToCartCommandHandler,
            create_cart_handler::CreateCartCommandHandler,
        },
        event::{
            added_to_cart_event::AddedToCartEvent, added_to_cart_handler::AddedToCartEventHandler,
            created_cart_event::CreatedCartEvent, created_cart_handler::CreatedCartEventHandler,
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

    // Used for mapping event types onto human-readable mongodb entries.
    let event_kinds = hashmap! {
        TypeId::of::<AddedToCartEvent>() => "addToCart".to_string(),
        TypeId::of::<CreatedCartEvent>() => "createdCart".to_string()
    };

    let event_store = Arc::new(EventStore::new(event_kinds).await);

    let queue = Arc::new(Mutex::new(MessageQueue::new(event_store.clone())));
    let mut registry = HandlerRegistry::new(&event_store);

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
        .layer(Extension(queue));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
