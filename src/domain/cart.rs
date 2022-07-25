// use super::offering::Offering;

#[derive(Debug)]
pub struct Item {
    offering: String, // TODO: Offering instead of String
    quantity: u8,
}

#[derive(Debug)]
pub struct Cart {
    items: Vec<String>,
}

impl Cart {
    pub fn items(&self) -> &Vec<String> {
        &self.items
    }
}

impl Cart {
    pub fn new(items: Vec<String>) -> Cart {
        Cart { items }
    }
}
