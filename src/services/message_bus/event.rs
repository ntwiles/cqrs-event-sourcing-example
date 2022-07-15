use std::fmt;

use chrono::{offset::Utc, DateTime};
use serde::Serialize;
use uuid::Uuid;

use crate::application::event::added_to_cart_event::AddedToCartEvent;
use crate::application::event::created_cart_event::CreatedCartEvent;

// TODO: Do we really want to exhaustively declare all event types from with the services module?
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize)]
pub enum EventType {
    AddedToCart(AddedToCartEvent),
    CreatedCart(CreatedCartEvent),
}

impl fmt::Display for EventType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

// impl PartialEq for EventType {
//     fn eq(&self, other: &EventType) -> bool {
//         self.to_string() == other.to_string()
//     }
// }

// TODO: Consider moving this to ./services/
#[derive(Debug, Serialize)]
pub struct Event {
    id: Uuid,
    data: EventType,
    when: DateTime<Utc>,
}

impl Event {
    pub fn new(data: EventType) -> Event {
        Event {
            data,
            id: Uuid::new_v4(),
            when: Utc::now(),
        }
    }

    pub fn data(&self) -> &EventType {
        &self.data
    }
}
