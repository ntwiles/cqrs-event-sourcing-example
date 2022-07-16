use serde::Deserialize;
use uuid::Uuid;

use std::any::{Any, TypeId};

use crate::services::message_bus::message::{Message, MessageData};

#[derive(Deserialize)]
pub struct AddToCartCommandData {
    cart_id: Uuid,
    customer_id: Uuid,
    offering_id: Uuid,
    quantity: u8,
}

impl AddToCartCommandData {
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

impl MessageData for AddToCartCommandData {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Deserialize)]
pub struct AddToCartCommand {
    data: AddToCartCommandData,
}

impl Message for AddToCartCommand {
    fn code(&self) -> TypeId {
        TypeId::of::<AddToCartCommand>()
    }

    fn data(&self) -> &dyn MessageData {
        &self.data
    }
}
