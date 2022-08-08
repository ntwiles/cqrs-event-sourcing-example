use bson::oid;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Product {
    _id: oid::ObjectId,
    name: String,
    price: i32,
}
