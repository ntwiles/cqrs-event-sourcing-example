// use super::offering::Offering;

pub struct Item {
    offering: String, // TODO: Offering instead of String
    quantity: u8,
}

pub struct Cart {
    items: Vec<Item>,
}

impl Cart {
    pub fn new() -> Cart {
        Cart { items: Vec::new() }
    }
}
