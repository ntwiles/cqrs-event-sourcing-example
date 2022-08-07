use axum::{http::StatusCode, Extension, Json};

use std::sync::Arc;

use crate::{application::query::product::ProductStore, domain::product::Product};

pub async fn read(
    Extension(product_store): Extension<Arc<ProductStore>>,
) -> Result<Json<Vec<Product>>, StatusCode> {
    product_store
        .index()
        .await
        .map(|c| Json(c))
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}
