use bson::oid;
use serde::Serialize;

use std::any::Any;

use crate::infrastructure::message_bus::{event::EventData, message::MessageData};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize)]
pub struct AddedToCartEvent {
    customer_id: oid::ObjectId,
    cart_id: oid::ObjectId,
    offering_id: oid::ObjectId,
    quantity: u8,
}

impl AddedToCartEvent {
    pub fn new(
        cart_id: oid::ObjectId,
        customer_id: oid::ObjectId,
        offering_id: oid::ObjectId,
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

impl MessageData for AddedToCartEvent {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl EventData for AddedToCartEvent {
    fn kind(&self) -> String {
        "addedToCart".to_string()
    }
}
