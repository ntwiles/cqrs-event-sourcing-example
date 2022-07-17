use axum::{http::StatusCode, response::IntoResponse, Extension, Json};

use std::sync::{Arc, Mutex};

use crate::application::command::{
    add_to_cart_command::AddToCartCommand, create_cart_command::CreateCartCommand,
};
use crate::services::message_bus::{message::Message, queue::MessageQueue};

pub async fn create<'a>(
    Json(command): Json<CreateCartCommand>,
    Extension(messsage_queue): Extension<Arc<Mutex<MessageQueue>>>,
) -> impl IntoResponse {
    let message = Message::new_command(command);
    messsage_queue.lock().unwrap().send(message);
    StatusCode::CREATED
}

pub async fn update(
    Json(command): Json<AddToCartCommand>,
    Extension(messsage_queue): Extension<Arc<Mutex<MessageQueue>>>,
) -> impl IntoResponse {
    let message = Message::new_command(command);
    messsage_queue.lock().unwrap().send(message);
    StatusCode::OK
}
