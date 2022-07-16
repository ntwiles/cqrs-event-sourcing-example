use axum::{http::StatusCode, response::IntoResponse, Extension, Json};

use crate::application::command::{
    add_to_cart_command::AddToCartCommand, create_cart_command::CreateCartCommand,
};
use crate::services::message_bus::bus::MessageBus;

pub async fn insert<'a>(
    Json(add_to_cart_command): Json<AddToCartCommand>,
    Extension(message_bus): Extension<&'a MessageBus>,
) -> impl IntoResponse {
    let create_cart_command = CreateCartCommand::new(add_to_cart_command.customer_id().clone());

    // CreateCartCommandHandler::new()
    //     .handle(create_cart_command)
    //     .await;

    // AddToCartCommandHandler::new()
    //     .handle(add_to_cart_command)
    //     .await;

    message_bus.send(Box::new(create_cart_command)).await;

    StatusCode::CREATED
}
