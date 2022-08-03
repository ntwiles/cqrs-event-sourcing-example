use axum::{http::StatusCode, response::IntoResponse, Extension, Json};
use futures::lock::Mutex;

use std::sync::Arc;

use crate::{
    application::{
        command::{add_to_cart_command::AddToCartCommand, create_cart_command::CreateCartCommand},
        query::cart_query::CartQuery,
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
    match bson::to_bson(&command) {
        Ok(data) => {
            messsage_queue
                .lock()
                .await
                .send_command(CommandKind::CreateCart, data);

            return StatusCode::CREATED;
        }
        Err(_e) => return StatusCode::BAD_REQUEST,
    }
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
    Json(query): Json<CartQuery>,
    Extension(cart_store): Extension<Arc<CartStore>>,
) -> impl IntoResponse {
    let result = cart_store.get(query.cart_id().clone()).await;
    (StatusCode::OK, Json(result))
}
