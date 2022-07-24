use bson::oid;
use serde::Deserialize;

use std::any::Any;

use crate::infrastructure::message_bus::message::MessageData;

#[derive(Debug, Deserialize)]
pub struct CreateCartCommand {
    customer_id: oid::ObjectId,
}

impl CreateCartCommand {
    // pub fn new(customer_id: oid::ObjectId) -> CreateCartCommand {
    //     CreateCartCommand { customer_id }
    // }

    pub fn customer_id(&self) -> &oid::ObjectId {
        &self.customer_id
    }
}

impl MessageData for CreateCartCommand {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
