use axum::{extract::Path, http::StatusCode, Extension, Json};
use bson::oid;
use futures::lock::Mutex;

use std::sync::Arc;

use crate::{
    application::command::{
        add_to_cart_command::AddToCartCommand, create_cart_command::CreateCartCommand,
    },
    domain::cart::Cart,
    infrastructure::{
        message_bus::{command_kind::CommandKind, queue::MessageQueue},
        read_stores::cart::CartStore,
    },
};

pub async fn create(
    Json(command): Json<CreateCartCommand>,
    Extension(messsage_queue): Extension<Arc<Mutex<MessageQueue>>>,
) -> Result<StatusCode, StatusCode> {
    let data = bson::to_bson(&command).map_err(|_| StatusCode::BAD_REQUEST)?;

    messsage_queue
        .lock()
        .await
        .send_command(CommandKind::CreateCart, data);

    Ok(StatusCode::CREATED)
}

pub async fn update(
    Json(command): Json<AddToCartCommand>,
    Extension(messsage_queue): Extension<Arc<Mutex<MessageQueue>>>,
) -> Result<StatusCode, StatusCode> {
    let data = bson::to_bson(&command).map_err(|_| StatusCode::BAD_REQUEST)?;

    messsage_queue
        .lock()
        .await
        .send_command(CommandKind::AddToCart, data);

    Ok(StatusCode::OK)
}

pub async fn read(
    Path(cart_id): Path<String>,
    Extension(cart_store): Extension<Arc<CartStore>>,
) -> Result<Json<Cart>, StatusCode> {
    let cart_id = oid::ObjectId::parse_str(cart_id).map_err(|_| StatusCode::BAD_REQUEST)?;

    cart_store
        .get(cart_id)
        .await
        .map(|c| Json(c))
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}
