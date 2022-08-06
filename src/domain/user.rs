use bson::oid;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct User {
    curent_cart: Option<oid::ObjectId>,
    first_name: String,
    last_name: String,
}

impl User {
    pub fn new() -> Self {
        User {
            curent_cart: None,
            first_name: String::new(),
            last_name: String::new(),
        }
    }
}
