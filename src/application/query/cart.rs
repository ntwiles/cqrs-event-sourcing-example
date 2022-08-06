use async_trait::async_trait;
use bson::oid;
use mongodb::Cursor;

use std::sync::Arc;

use crate::application::event::added_to_cart_event::AddedToCartEvent;
use crate::domain::cart::{Cart, Item};
use crate::infrastructure::{
    message_bus::event_kind::EventKind,
    persistence::events::{Event, EventService},
};

use super::replay::Replay;

pub struct CartStore {
    event_service: Arc<EventService>,
}

impl CartStore {
    pub fn new(event_service: Arc<EventService>) -> Self {
        Self { event_service }
    }

    pub async fn get(&self, cart_id: oid::ObjectId) -> Result<Cart, mongodb::error::Error> {
        let events = self.event_service.find_events(cart_id).await?;
        Self::replay(events).await
    }
}

#[async_trait]
impl Replay<Cart> for CartStore {
    async fn replay(mut events: Cursor<Event>) -> Result<Cart, mongodb::error::Error> {
        let mut items = Vec::<Item>::new();

        while events.advance().await? {
            let event = events.deserialize_current()?;

            match event.kind() {
                EventKind::CreatedCart => (),
                EventKind::AddedToCart => {
                    let data = bson::from_bson::<AddedToCartEvent>(event.data().clone())?;
                    items.push(Item::new(&data.offering_id().to_string(), data.quantity()));
                }
            }
        }

        Ok(Cart::new(items))
    }
}
