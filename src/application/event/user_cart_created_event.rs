use bson::oid;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize)]
pub struct UserCartCreatedEvent {
    cart_id: oid::ObjectId,
}

impl UserCartCreatedEvent {
    pub fn new(cart_id: oid::ObjectId) -> Self {
        Self { cart_id }
    }

    // pub fn cart_id(&self) -> oid::ObjectId {
    //     self.cart_id
    // }
}
