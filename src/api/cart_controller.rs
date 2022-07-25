use axum::{http::StatusCode, response::IntoResponse, Extension, Json};
use bson::Bson;
use futures::lock::Mutex;

use std::sync::Arc;

use crate::application::command::{
    add_to_cart_command::AddToCartCommand, create_cart_command::CreateCartCommand,
};
use crate::infrastructure::message_bus::queue::MessageQueue;

pub async fn create<'a>(
    Json(command): Json<CreateCartCommand>,
    Extension(messsage_queue): Extension<Arc<Mutex<MessageQueue>>>,
) -> impl IntoResponse {
    let data = bson::to_bson(&command).unwrap();

    messsage_queue
        .lock()
        .await
        .send_command("createCart".to_string(), data);
    StatusCode::CREATED
}

pub async fn update(
    Json(command): Json<AddToCartCommand>,
    Extension(messsage_queue): Extension<Arc<Mutex<MessageQueue>>>,
) -> impl IntoResponse {
    let data = bson::to_bson(&command).unwrap();

    messsage_queue
        .lock()
        .await
        .send_command("addToCart".to_string(), data);
    StatusCode::OK
}

pub async fn read(// Json(command): Json<AddToCartCommand>,
) -> impl IntoResponse {
    (StatusCode::OK, "a response!")
}
