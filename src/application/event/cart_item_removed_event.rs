use serde::{Deserialize, Serialize};

use crate::domain::product::Product;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CartItemRemovedEvent {
    pub product: Product,
}

impl CartItemRemovedEvent {
    pub fn new(product: Product) -> Self {
        Self { product }
    }
}
