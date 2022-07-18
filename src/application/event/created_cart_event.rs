use bson::oid;
use serde::Serialize;

use std::any::Any;

use crate::services::message_bus::{event::EventData, message::MessageData};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize)]
pub struct CreatedCartEvent {
    customer_id: oid::ObjectId,
}

impl CreatedCartEvent {
    pub fn new(customer_id: oid::ObjectId) -> CreatedCartEvent {
        CreatedCartEvent { customer_id }
    }
}

impl MessageData for CreatedCartEvent {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl EventData for CreatedCartEvent {
    fn kind(&self) -> String {
        "createdCart".to_string()
    }
}
