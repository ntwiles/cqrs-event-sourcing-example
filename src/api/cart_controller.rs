use axum::{http::StatusCode, Extension, Json};
use bson::oid;
use futures::lock::Mutex;
use serde::Deserialize;

use std::sync::Arc;

use crate::{
    application::{
        command::cart_add_item_command::CartAddItemCommand,
        event::user_cart_created_event::UserCartCreatedEvent,
        query::{cart::CartStore, user::UserStore},
    },
    domain::cart::Cart,
    infrastructure::message_bus::{
        command_kind::CommandKind, event_kind::EventKind, queue::MessageQueue,
    },
};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CartUpdateRequestBody {
    product_id: oid::ObjectId,
    quantity: u8,
}

pub async fn update(
    Json(req): Json<CartUpdateRequestBody>,
    Extension(messsage_queue): Extension<Arc<Mutex<MessageQueue>>>,
    Extension(user_store): Extension<Arc<UserStore>>,
) -> Result<StatusCode, StatusCode> {
    // Simulate authenticated user.
    let user_id = oid::ObjectId::parse_str("62eee62cee48789afbd1354a").unwrap();

    let customer = user_store
        .get(user_id)
        .await
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    let cart_id = customer.current_cart.unwrap_or(oid::ObjectId::new());

    if customer.current_cart.is_none() {
        raise_cart_created_event(user_id, cart_id, messsage_queue.clone()).await;
    };

    let command = CartAddItemCommand {
        cart_id,
        product_id: req.product_id,
        quantity: req.quantity,
    };

    let data = bson::to_bson(&command).map_err(|_| StatusCode::BAD_REQUEST)?;

    messsage_queue
        .lock()
        .await
        .send_command(CommandKind::AddToCart, data);

    Ok(StatusCode::OK)
}

pub async fn read(
    Extension(cart_store): Extension<Arc<CartStore>>,
    Extension(user_store): Extension<Arc<UserStore>>,
) -> Result<Json<Cart>, StatusCode> {
    // Simulate authenticated user.
    let user_id = oid::ObjectId::parse_str("62eee62cee48789afbd1354a").unwrap();

    let customer = user_store
        .get(user_id)
        .await
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    if let Some(cart_id) = customer.current_cart {
        cart_store
            .get(cart_id)
            .await
            .map(|c| Json(c))
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

async fn raise_cart_created_event(
    user_id: oid::ObjectId,
    cart_id: oid::ObjectId,
    message_queue: Arc<Mutex<MessageQueue>>,
) {
    let event = bson::to_bson(&UserCartCreatedEvent::new(cart_id)).unwrap();

    message_queue
        .lock()
        .await
        .raise_event(user_id, EventKind::UserCartCreated, event)
        .await
        .unwrap()
}
