use bson::oid;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct User {
    current_cart: Option<oid::ObjectId>,
    first_name: String,
    last_name: String,
}

impl User {
    pub fn new() -> Self {
        User {
            current_cart: None,
            first_name: String::new(),
            last_name: String::new(),
        }
    }

    pub fn current_cart(&self) -> Option<oid::ObjectId> {
        self.current_cart.clone()
    }
}
