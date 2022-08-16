use axum::{
    http::Method,
    routing::{get, post},
    Extension, Router,
};
use dotenv::dotenv;
use futures::lock::Mutex;
use tower_http::cors::{Any, CorsLayer};

use std::{net::SocketAddr, sync::Arc};

use crate::{
    api::{cart_controller, cart_events_controller, product_controller, user_controller},
    application::{
        command::{
            cart_add_item_handler::CartAddItemHandler,
            cart_remove_item_handler::CartRemoveItemHandler,
        },
        event::{
            cart_item_added_handler::CartItemAddedHandler,
            user_cart_created_handler::UserCartCreatedHandler,
        },
        query::{cart::CartStore, product::ProductStore, user::UserStore},
    },
    infrastructure::{
        message_bus::{queue::MessageQueue, registry::HandlerRegistry, start_message_loop},
        persistence::{events::EventService, products::ProductService},
    },
};

mod api;
mod application;
mod domain;
mod infrastructure;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let event_service = Arc::new(EventService::new().await);
    let product_service = Arc::new(ProductService::new().await);

    let cart_store = Arc::new(CartStore::new(event_service.clone()));
    let product_store = Arc::new(ProductStore::new(product_service.clone()));
    let user_store = Arc::new(UserStore::new(event_service.clone()));

    let bus = Arc::new(Mutex::new(MessageQueue::new(event_service.clone())));
    let mut registry = HandlerRegistry::new(&event_service);

    // commands
    registry.add(Box::new(CartAddItemHandler::new(
        bus.clone(),
        product_store.clone(),
    )));
    registry.add(Box::new(CartRemoveItemHandler::new(
        bus.clone(),
        product_store.clone(),
    )));

    // events
    registry.add(Box::new(CartItemAddedHandler::new()));
    registry.add(Box::new(UserCartCreatedHandler::new()));

    start_message_loop(bus.clone(), registry);

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/cart", get(cart_controller::read))
        .route("/cart", post(cart_controller::update))
        .route("/cart/remove", post(cart_controller::delete))
        .route("/cart-events", get(cart_events_controller::read))
        .route("/products", get(product_controller::read))
        .route("/user/:user_id", get(user_controller::read))
        .layer(cors)
        .layer(Extension(bus))
        .layer(Extension(cart_store))
        .layer(Extension(user_store))
        .layer(Extension(product_store));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
