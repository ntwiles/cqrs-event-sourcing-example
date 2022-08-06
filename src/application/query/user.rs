use async_trait::async_trait;
use bson::oid;
use mongodb::Cursor;

use std::sync::Arc;

use crate::domain::user::User;
use crate::infrastructure::persistence::events::{Event, EventService};

use super::replay::Replay;

pub struct UserStore {
    event_service: Arc<EventService>,
}

impl UserStore {
    pub fn new(event_service: Arc<EventService>) -> Self {
        Self { event_service }
    }

    pub async fn get(&self, user_id: oid::ObjectId) -> Result<User, mongodb::error::Error> {
        let events = self.event_service.find_events(user_id).await?;
        Self::replay(events).await
    }
}

#[async_trait]
impl Replay<User> for UserStore {
    async fn replay(mut events: Cursor<Event>) -> Result<User, mongodb::error::Error> {
        let user = User::new();

        while events.advance().await? {
            let event = events.deserialize_current()?;

            match event.kind() {
                _ => todo!(),
            }
        }

        Ok(user)
    }
}
