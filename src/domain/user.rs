use bson::oid;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct User {
    pub current_cart: Option<oid::ObjectId>,
    pub first_name: String,
    pub last_name: String,
}

impl User {
    pub fn new() -> Self {
        User {
            current_cart: None,
            first_name: String::new(),
            last_name: String::new(),
        }
    }
}
