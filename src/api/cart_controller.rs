use axum::{http::StatusCode, response::IntoResponse, Extension, Json};

use std::sync::{Arc, Mutex};

use crate::application::command::{
    add_to_cart_command::AddToCartCommand, create_cart_command::CreateCartCommand,
};
use crate::services::message_bus::queue::MessageQueue;

pub async fn insert<'a>(
    Json(add_to_cart_command): Json<AddToCartCommand>,
    Extension(messsage_queue): Extension<Arc<Mutex<MessageQueue>>>,
) -> impl IntoResponse {
    let create_cart_command = CreateCartCommand::new(add_to_cart_command.customer_id().clone());

    messsage_queue
        .lock()
        .unwrap()
        .send(Box::new(create_cart_command));

    StatusCode::CREATED
}
