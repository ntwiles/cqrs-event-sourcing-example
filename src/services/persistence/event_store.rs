use bson::oid;
use chrono::{offset::Utc, DateTime};
use dotenv_codegen::dotenv;
use mongodb::{options::ClientOptions, Client};
use serde::Serialize;

use std::{any, fmt::Debug};

use crate::services::message_bus::event::EventData;

#[derive(Debug, Serialize)]
pub struct Event<T: EventData> {
    data: T,
    id: oid::ObjectId,
    kind: String,
    when: DateTime<Utc>,
}

impl<T: EventData> Event<T> {
    pub fn new<U: EventData>(data: U) -> Event<U> {
        Event {
            data,
            id: oid::ObjectId::new(),
            kind: any::type_name::<T>().to_owned(),
            when: Utc::now(),
        }
    }
}

pub struct EventStore {
    db: mongodb::Database,
}

impl EventStore {
    pub async fn new() -> EventStore {
        let client_options = ClientOptions::parse(dotenv!("MONGODB_CONNECTION_STRING"))
            .await
            .unwrap();

        let db = Client::with_options(client_options)
            .unwrap()
            .database("cqrs-event-sourcing");

        EventStore { db }
    }

    fn collection<T: EventData>(&self) -> mongodb::Collection<Event<T>> {
        self.db.collection::<Event<T>>("events")
    }

    pub async fn write_event<T: EventData>(&self, data: &T) {
        println!("Writing event!");
        let event = Event::<T>::new(*data);
        let result = self.collection().insert_one(event, None).await;
        println!("{:?}", result);
    }
}
