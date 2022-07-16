use serde::Serialize;
use uuid::Uuid;

use std::any::{Any, TypeId};

use crate::services::message_bus::message::{Message, MessageData};

#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize)]
pub struct CreatedCartEventData {
    customer_id: Uuid,
}

impl MessageData for CreatedCartEventData {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize)]
pub struct CreatedCartEvent {
    data: CreatedCartEventData,
}

impl CreatedCartEvent {
    pub fn new(customer_id: Uuid) -> CreatedCartEvent {
        CreatedCartEvent {
            data: CreatedCartEventData { customer_id },
        }
    }
}

impl Message for CreatedCartEvent {
    fn code(&self) -> TypeId {
        TypeId::of::<CreatedCartEvent>()
    }

    fn data(&self) -> &dyn MessageData {
        &self.data
    }
}
