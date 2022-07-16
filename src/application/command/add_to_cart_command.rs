use serde::Deserialize;
use uuid::Uuid;

use std::any::Any;

use crate::services::message_bus::message::MessageData;

#[derive(Deserialize)]
pub struct AddToCartCommand {
    cart_id: Uuid,
    customer_id: Uuid,
    offering_id: Uuid,
    quantity: u8,
}

impl AddToCartCommand {
    pub fn cart_id(&self) -> &Uuid {
        &self.cart_id
    }

    pub fn customer_id(&self) -> &Uuid {
        &self.customer_id
    }

    pub fn offering_id(&self) -> &Uuid {
        &self.offering_id
    }

    pub fn quantity(&self) -> &u8 {
        &self.quantity
    }
}

impl MessageData for AddToCartCommand {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
