use bson::oid;
use serde::{Deserialize, Serialize};

use std::any::Any;

use crate::infrastructure::message_bus::message::MessageData;

#[derive(Debug, Deserialize, Serialize)]
pub struct AddToCartCommand {
    customer_id: oid::ObjectId,
    offering_id: oid::ObjectId,
    quantity: u8,
}

impl AddToCartCommand {
    pub fn customer_id(&self) -> &oid::ObjectId {
        &self.customer_id
    }

    pub fn offering_id(&self) -> &oid::ObjectId {
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
