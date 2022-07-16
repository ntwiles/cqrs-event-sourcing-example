use serde::Deserialize;
use uuid::Uuid;

use std::any::TypeId;

use crate::services::message_bus::message::Message;

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

impl Message for CreateCartCommand {
    fn message_type(&self) -> TypeId {
        TypeId::of::<CreateCartCommand>()
    }
}
