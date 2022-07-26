use serde::Serialize;

use crate::infrastructure::message_bus::event::EventData;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize)]
pub struct CreatedCartEvent {}

impl EventData for CreatedCartEvent {
    fn kind(&self) -> String {
        "createdCart".to_string()
    }
}
