use async_trait::async_trait;
use bson::oid;
use mongodb::Cursor;

use std::sync::Arc;

use crate::application::event::added_to_cart_event::AddedToCartEvent;
use crate::domain::cart::{Cart, Item};
use crate::infrastructure::persistence::events::{Event, EventService};

use super::replay::Replay;

pub struct CartStore {
    event_service: Arc<EventService>,
}

impl CartStore {
    pub fn new(event_service: Arc<EventService>) -> CartStore {
        CartStore { event_service }
    }
    pub async fn get(&self, customer_id: oid::ObjectId) -> Cart {
        let events = self.event_service.find_events(customer_id).await.unwrap();

        CartStore::replay(events).await.unwrap()
    }
}

#[async_trait]
impl Replay<Cart> for CartStore {
    async fn replay(mut events: Cursor<Event>) -> Result<Cart, mongodb::error::Error> {
        let mut items = Vec::<Item>::new();

        while events.advance().await? {
            let event = events.deserialize_current().unwrap();

            match event.kind() {
                "createdCart" => (),
                "addedToCart" => {
                    let data = bson::from_bson::<AddedToCartEvent>(event.data().clone()).unwrap();
                    items.push(Item::new(&data.offering_id().to_string(), data.quantity()));
                }
                _ => panic!(),
            }
        }

        Ok(Cart::new(items))
    }
}
