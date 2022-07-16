use serde::Serialize;
use uuid::Uuid;

use std::any::{Any, TypeId};

use crate::services::message_bus::message::{Message, MessageData};

#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize)]
pub struct AddedToCartEventData {
    customer_id: Uuid,
    cart_id: Uuid,
    offering_id: Uuid,
    quantity: u8,
}

impl MessageData for AddedToCartEventData {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize)]
pub struct AddedToCartEvent {
    data: AddedToCartEventData,
}

impl AddedToCartEvent {
    pub fn new(
        cart_id: Uuid,
        customer_id: Uuid,
        offering_id: Uuid,
        quantity: u8,
    ) -> AddedToCartEvent {
        AddedToCartEvent {
            data: AddedToCartEventData {
                cart_id,
                customer_id,
                offering_id,
                quantity,
            },
        }
    }
}

impl Message for AddedToCartEvent {
    fn code(&self) -> TypeId {
        TypeId::of::<AddedToCartEvent>()
    }

    fn data(&self) -> &dyn MessageData {
        &self.data
    }
}
