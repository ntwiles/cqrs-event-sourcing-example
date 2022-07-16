use serde::Serialize;
use uuid::Uuid;

use std::any::Any;

use crate::services::message_bus::message::MessageData;

#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize)]
pub struct CreatedCartEvent {
    customer_id: Uuid,
}

impl CreatedCartEvent {
    pub fn new(customer_id: Uuid) -> CreatedCartEvent {
        CreatedCartEvent { customer_id }
    }
}

impl MessageData for CreatedCartEvent {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
