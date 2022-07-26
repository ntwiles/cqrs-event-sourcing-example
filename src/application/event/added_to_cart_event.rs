use bson::oid;
use serde::{Deserialize, Serialize};

use std::any::Any;

use crate::infrastructure::message_bus::{event::EventData, message::MessageData};

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
