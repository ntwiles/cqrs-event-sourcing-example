use bson::oid;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct CartAddItemCommand {
    pub cart_id: oid::ObjectId,
    pub product_id: oid::ObjectId,
    pub quantity: u8,
}
