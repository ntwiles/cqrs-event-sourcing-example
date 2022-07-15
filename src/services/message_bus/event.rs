use chrono::{offset::Utc, DateTime};
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct Event {
    id: Uuid,
    when: DateTime<Utc>,
}

impl Event {
    pub fn new() -> Event {
        Event {
            id: Uuid::new_v4(),
            when: Utc::now(),
        }
    }
}
