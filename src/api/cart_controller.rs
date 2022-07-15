use axum::{http::StatusCode, response::IntoResponse, Json};

use crate::application::command::{
    add_to_cart_command::AddToCartCommand, create_cart_command::CreateCartCommand,
};

pub async fn insert(Json(add_to_cart_command): Json<AddToCartCommand>) -> impl IntoResponse {
    let create_cart_command = CreateCartCommand::new(add_to_cart_command.customer_id().clone());

    // CreateCartCommandHandler::new()
    //     .handle(create_cart_command)
    //     .await;

    // AddToCartCommandHandler::new()
    //     .handle(add_to_cart_command)
    //     .await;

    StatusCode::CREATED
}
