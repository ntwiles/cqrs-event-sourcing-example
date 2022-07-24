use serde::Serialize;

use std::any::Any;

use crate::infrastructure::message_bus::{event::EventData, message::MessageData};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize)]
pub struct CreatedCartEvent {}

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
