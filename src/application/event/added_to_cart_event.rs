use serde::Serialize;
use uuid::Uuid;

use std::any::Any;

use crate::services::message_bus::{event::EventData, message::MessageData};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize)]
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

impl MessageData for AddedToCartEvent {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl EventData for AddedToCartEvent {}
