use serde::Deserialize;
use uuid::Uuid;

use std::any::Any;

use crate::services::message_bus::message::MessageData;

#[derive(Deserialize)]
pub struct CreateCartCommand {
    customer_id: Uuid,
}

impl CreateCartCommand {
    pub fn new(customer_id: Uuid) -> CreateCartCommand {
        CreateCartCommand { customer_id }
    }

    pub fn customer_id(&self) -> &Uuid {
        &self.customer_id
    }
}

impl MessageData for CreateCartCommand {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
