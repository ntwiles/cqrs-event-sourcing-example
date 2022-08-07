use serde::Serialize;

use super::product::Product;

#[derive(Debug, Serialize)]
pub struct Item {
    product: Product,
    quantity: u8,
}

impl Item {
    pub fn new(product: Product, quantity: u8) -> Item {
        Item { product, quantity }
    }
}

#[derive(Debug, Serialize)]
pub struct Cart {
    pub items: Vec<Item>,
}

impl Cart {
    pub fn new() -> Cart {
        Cart { items: Vec::new() }
    }
}
