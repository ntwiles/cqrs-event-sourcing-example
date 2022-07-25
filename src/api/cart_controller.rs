use axum::{http::StatusCode, response::IntoResponse, Extension, Json};
use futures::lock::Mutex;

use std::sync::Arc;

use crate::application::{
    command::{add_to_cart_command::AddToCartCommand, create_cart_command::CreateCartCommand},
    query::{cart_query::CartQuery, cart_store::CartStore},
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

pub async fn read(
    Json(query): Json<CartQuery>,
    Extension(cart_store): Extension<Arc<CartStore>>,
) -> impl IntoResponse {
    let result = cart_store.get(query.customer_id().clone()).await;
    println!("Result: {:?}", result);
    (StatusCode::OK, "a response!")
}
