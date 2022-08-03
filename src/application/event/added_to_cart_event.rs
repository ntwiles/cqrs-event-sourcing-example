use bson::oid;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct AddedToCartEvent {
    offering_id: oid::ObjectId,
    quantity: u8,
}

impl AddedToCartEvent {
    pub fn new(offering_id: oid::ObjectId, quantity: u8) -> AddedToCartEvent {
        AddedToCartEvent {
            offering_id,
            quantity,
        }
    }

    pub fn offering_id(&self) -> oid::ObjectId {
        self.offering_id
    }

    pub fn quantity(&self) -> u8 {
        self.quantity
    }
}
