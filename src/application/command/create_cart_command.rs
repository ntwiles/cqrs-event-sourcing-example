use serde::Deserialize;
use uuid::Uuid;

use std::any::{Any, TypeId};

use crate::services::message_bus::message::{Message, MessageData};

#[derive(Deserialize)]
pub struct CreateCartCommandData {
    customer_id: Uuid,
}

impl CreateCartCommandData {
    pub fn customer_id(&self) -> &Uuid {
        &self.customer_id
    }
}

impl MessageData for CreateCartCommandData {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Deserialize)]
pub struct CreateCartCommand {
    data: CreateCartCommandData,
}

impl CreateCartCommand {
    pub fn new(customer_id: Uuid) -> CreateCartCommand {
        CreateCartCommand {
            data: CreateCartCommandData { customer_id },
        }
    }
}

impl Message for CreateCartCommand {
    fn code(&self) -> TypeId {
        TypeId::of::<CreateCartCommand>()
    }

    fn data(&self) -> &dyn MessageData {
        &self.data
    }
}
