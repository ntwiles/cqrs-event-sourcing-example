use chrono::{offset::Utc, DateTime};
use dotenv_codegen::dotenv;
use mongodb::{options::ClientOptions, Client};
use serde::Serialize;
use uuid::Uuid;

use std::fmt::Debug;

use crate::services::message_bus::event::EventData;

#[derive(Debug, Serialize)]
pub struct Event<T: EventData> {
    id: Uuid,
    when: DateTime<Utc>,
    data: T,
}

impl<T: EventData> Event<T> {
    pub fn new<U: EventData>(data: U) -> Event<U> {
        Event {
            id: Uuid::new_v4(),
            when: Utc::now(),
            data,
        }
    }
}

pub struct EventStore {}

impl EventStore {
    pub fn new() -> EventStore {
        EventStore {}
    }

    pub async fn write_event<T: EventData>(&self, data: &T) {
        let client_options = ClientOptions::parse(dotenv!("MONGODB_CONNECTION_STRING"))
            .await
            .unwrap();

        let db = Client::with_options(client_options)
            .unwrap()
            .database("cqrs-event-sourcing");

        let collection = db.collection::<Event<T>>("event-writes");

        let event: Event<T> = Event::<T>::new(*data);

        let result = collection.insert_one(event, None).await;

        println!("{:?}", result);
    }
}
