pub mod added_to_cart_event;
pub mod created_cart_event;

use chrono::{offset::Utc, DateTime};
use serde::Serialize;
use uuid::Uuid;

use added_to_cart_event::AddedToCartEventData;
use created_cart_event::CreatedCartEventData;

#[derive(Debug, Serialize)]
pub enum EventType {
    AddedToCart(AddedToCartEventData),
    CreatedCart(CreatedCartEventData),
}

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
