use bson::oid;
use serde::Deserialize;

use std::any::Any;

use crate::services::message_bus::message::MessageData;

#[derive(Debug, Deserialize)]
pub struct CreateCartCommand {
    customer_id: oid::ObjectId,
}

impl CreateCartCommand {
    pub fn customer_id(&self) -> &oid::ObjectId {
        &self.customer_id
    }
}

impl MessageData for CreateCartCommand {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
