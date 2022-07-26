use serde::Serialize;

// use super::offering::Offering;

#[derive(Debug, Serialize)]
pub struct Item {
    offering: String, // TODO: Offering instead of String
    quantity: u8,
}

impl Item {
    pub fn new(offering: &str, quantity: u8) -> Item {
        Item {
            offering: String::from(offering),
            quantity,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct Cart {
    items: Vec<Item>,
}

impl Cart {
    // pub fn items(&self) -> &Vec<Item> {
    //     &self.items
    // }
}

impl Cart {
    pub fn new(items: Vec<Item>) -> Cart {
        Cart { items }
    }
}
