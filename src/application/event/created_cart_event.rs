use serde::Serialize;
use uuid::Uuid;

use crate::services::message_bus::message::Message;

#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize)]
pub struct CreatedCartEvent {
    customer_id: Uuid,
}

impl CreatedCartEvent {
    pub fn new(customer_id: Uuid) -> CreatedCartEvent {
        CreatedCartEvent { customer_id }
    }
}

impl Message for CreatedCartEvent {}
