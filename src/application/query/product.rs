use bson::oid;
use futures::TryStreamExt;

use std::sync::Arc;

use crate::domain::product::Product;
use crate::infrastructure::persistence::products::ProductService;

pub struct ProductStore {
    service: Arc<ProductService>,
}

impl ProductStore {
    pub fn new(service: Arc<ProductService>) -> Self {
        Self { service }
    }

    pub async fn index(&self) -> Result<Vec<Product>, mongodb::error::Error> {
        match self.service.find().await {
            Ok(cursor) => {
                let products = cursor.try_collect().await;
                Ok(products?)
            }
            Err(e) => Err(e),
        }
    }

    pub async fn get(&self, cart_id: oid::ObjectId) -> Result<Product, mongodb::error::Error> {
        self.service
            .find_by_id(cart_id)
            .await
            .map(|option| option.expect("Could not find product by _id.")) // TODO: Can we make the mongo driver error instead?
    }
}
