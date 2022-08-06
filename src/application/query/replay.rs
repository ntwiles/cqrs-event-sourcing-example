use async_trait::async_trait;
use mongodb::Cursor;

use crate::infrastructure::persistence::events::Event;

#[async_trait]
pub trait Replay<T> {
    async fn replay(events: Cursor<Event>) -> Result<T, mongodb::error::Error>;
}
