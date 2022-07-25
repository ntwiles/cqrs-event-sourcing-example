use bson::oid;
use mongodb::Cursor;

use std::sync::Arc;

use crate::domain::cart::Cart;
use crate::infrastructure::persistence::events::{Event, EventService};

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

    async fn replay(mut events: Cursor<Event>) -> Result<Cart, mongodb::error::Error> {
        let mut items = Vec::<String>::new();

        while events.advance().await? {
            let event = events.current();
            let kind = event.get_str("kind").unwrap();

            println!("{}", kind);

            match kind {
                "createdCart" => (),
                "addedToCart" => items.push("test".to_string()),
                _ => panic!(),
            }
        }

        Ok(Cart::new(items))
    }
}
