use serde::Serialize;
use uuid::Uuid;

use crate::services::message_bus::message::Message;

#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize)]
pub struct AddedToCartEvent {
    customer_id: Uuid,
    cart_id: Uuid,
    offering_id: Uuid,
    quantity: u8,
}

impl AddedToCartEvent {
    pub fn new(
        cart_id: Uuid,
        customer_id: Uuid,
        offering_id: Uuid,
        quantity: u8,
    ) -> AddedToCartEvent {
        AddedToCartEvent {
            cart_id,
            customer_id,
            offering_id,
            quantity,
        }
    }
}

impl Message for AddedToCartEvent {}
