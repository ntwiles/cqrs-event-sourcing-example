use axum::{http::StatusCode, response::IntoResponse, Extension, Json};
use futures::lock::Mutex;

use std::sync::Arc;

use crate::application::command::{
    add_to_cart_command::AddToCartCommand, create_cart_command::CreateCartCommand,
};
use crate::services::message_bus::queue::MessageQueue;

pub async fn create<'a>(
    Json(command): Json<CreateCartCommand>,
    Extension(messsage_queue): Extension<Arc<Mutex<MessageQueue>>>,
) -> impl IntoResponse {
    messsage_queue.lock().await.send_command(command);
    StatusCode::CREATED
}

pub async fn update(
    Json(command): Json<AddToCartCommand>,
    Extension(messsage_queue): Extension<Arc<Mutex<MessageQueue>>>,
) -> impl IntoResponse {
    messsage_queue.lock().await.send_command(command);
    StatusCode::OK
}

pub async fn read(// Json(command): Json<AddToCartCommand>,
) -> impl IntoResponse {
    // let message = Message::new_command(command);
    // messsage_queue.lock().await.send(message);
    (StatusCode::OK, "a response!")
}
