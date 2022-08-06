use axum::{extract::Path, http::StatusCode, response::IntoResponse, Extension, Json};
use bson::oid;
use futures::lock::Mutex;

use std::{collections::HashMap, sync::Arc};

use crate::{
    application::command::{
        add_to_cart_command::AddToCartCommand, create_cart_command::CreateCartCommand,
    },
    infrastructure::{
        message_bus::{command_kind::CommandKind, queue::MessageQueue},
        read_stores::cart::CartStore,
    },
};

pub async fn create(
    Json(command): Json<CreateCartCommand>,
    Extension(messsage_queue): Extension<Arc<Mutex<MessageQueue>>>,
) -> impl IntoResponse {
    let data = bson::to_bson(&command).unwrap();

    messsage_queue
        .lock()
        .await
        .send_command(CommandKind::CreateCart, data);

    return StatusCode::CREATED;
}

pub async fn update(
    Json(command): Json<AddToCartCommand>,
    Extension(messsage_queue): Extension<Arc<Mutex<MessageQueue>>>,
) -> impl IntoResponse {
    let data = bson::to_bson(&command).unwrap();

    messsage_queue
        .lock()
        .await
        .send_command(CommandKind::AddToCart, data);

    StatusCode::OK
}

pub async fn read(
    Path(params): Path<HashMap<String, String>>,
    Extension(cart_store): Extension<Arc<CartStore>>,
) -> impl IntoResponse {
    let cart_id = params.get("cart_id").unwrap();

    match oid::ObjectId::parse_str(cart_id) {
        Ok(cart_id) => {
            let result = cart_store.get(cart_id).await;

            (StatusCode::OK, Json(Some(result)))
        }
        Err(_) => (StatusCode::BAD_REQUEST, Json(None)),
    }
}
