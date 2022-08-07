use serde::{Deserialize, Serialize};

use crate::domain::product::Product;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CartItemAddedEvent {
    pub product: Product,
    pub quantity: u8,
}

impl CartItemAddedEvent {
    pub fn new(product: Product, quantity: u8) -> Self {
        Self { product, quantity }
    }
}
