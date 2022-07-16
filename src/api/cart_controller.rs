use axum::{http::StatusCode, response::IntoResponse, Extension, Json};

use std::sync::{Arc, Mutex};

use crate::application::command::{
    add_to_cart_command::AddToCartCommand, create_cart_command::CreateCartCommand,
};
use crate::services::message_bus::{message::Message, queue::MessageQueue};

pub async fn insert<'a>(
    Json(add_to_cart_command): Json<AddToCartCommand>,
    Extension(messsage_queue): Extension<Arc<Mutex<MessageQueue>>>,
) -> impl IntoResponse {
    let command = CreateCartCommand::new(add_to_cart_command.customer_id().clone());

    let message = Message::new::<CreateCartCommand>(command);

    messsage_queue.lock().unwrap().send(message);

    StatusCode::CREATED
}
