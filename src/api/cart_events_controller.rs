use axum::{http::StatusCode, Extension, Json};
use bson::oid;

use std::sync::Arc;

use crate::{
    application::query::{cart::CartStore, user::UserStore},
    infrastructure::persistence::events::Event,
};

pub async fn read(
    Extension(cart_store): Extension<Arc<CartStore>>,
    Extension(user_store): Extension<Arc<UserStore>>,
) -> Result<Json<Vec<Event>>, StatusCode> {
    // Simulate authenticated user.
    let user_id = oid::ObjectId::parse_str("62eee62cee48789afbd1354a").unwrap();

    let customer = user_store
        .get(user_id)
        .await
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    if let Some(cart_id) = customer.current_cart {
        cart_store
            .get_raw(cart_id)
            .await
            .map(|c| Json(c))
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}
