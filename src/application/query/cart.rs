use async_trait::async_trait;
use bson::oid;
use futures::TryStreamExt;
use mongodb::Cursor;

use std::sync::Arc;

use crate::application::event::{
    cart_item_added_event::CartItemAddedEvent, cart_item_removed_event::CartItemRemovedEvent,
};
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

    pub async fn get_raw(
        &self,
        cart_id: oid::ObjectId,
    ) -> Result<Vec<Event>, mongodb::error::Error> {
        let result = self.event_service.find_events(cart_id).await;

        match result {
            Ok(cursor) => cursor.try_collect().await,
            Err(e) => Err(e),
        }
    }
}

#[async_trait]
impl Replay<Cart> for CartStore {
    async fn replay(mut events: Cursor<Event>) -> Result<Cart, mongodb::error::Error> {
        let mut cart = Cart::new();

        while events.advance().await? {
            let event = events.deserialize_current()?;

            match event.kind() {
                EventKind::CartItemAdded => {
                    let data = bson::from_bson::<CartItemAddedEvent>(event.data().clone())?;
                    cart.items.push(Item::new(data.product, data.quantity));
                }
                EventKind::CartItemRemoved => {
                    let data = bson::from_bson::<CartItemRemovedEvent>(event.data().clone())?;
                    cart.items
                        .iter()
                        .position(|i| i.product == data.product)
                        .map(|e| cart.items.remove(e));
                }
                k => panic!(
                    "Events of kind {:?} should not be correlated with cart_id {}.",
                    k,
                    event.correlation_id()
                ),
            }
        }

        Ok(cart)
    }
}
