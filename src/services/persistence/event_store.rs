use bson::oid;
use chrono::{offset::Utc, DateTime};
use dotenv_codegen::dotenv;
use mongodb::{options::ClientOptions, Client};
use serde::Serialize;

use std::{any::TypeId, collections::HashMap, fmt::Debug};

use crate::services::message_bus::event::EventData;

#[derive(Debug, Serialize)]
pub struct Event<T: EventData> {
    data: T,
    id: oid::ObjectId,
    kind: String,
    when: DateTime<Utc>,
}

impl<T: EventData> Event<T> {
    pub fn new<U: EventData>(data: U, kind: String) -> Event<U> {
        Event {
            data,
            id: oid::ObjectId::new(),
            kind,
            when: Utc::now(),
        }
    }
}

pub struct EventStore {
    db: mongodb::Database,
    kinds: HashMap<TypeId, String>,
}

impl EventStore {
    pub async fn new(kinds: HashMap<TypeId, String>) -> EventStore {
        let client_options = ClientOptions::parse(dotenv!("MONGODB_CONNECTION_STRING"))
            .await
            .unwrap();

        let db = Client::with_options(client_options)
            .unwrap()
            .database("cqrs-event-sourcing");

        EventStore { db, kinds }
    }

    fn collection<T: EventData>(&self) -> mongodb::Collection<Event<T>> {
        self.db.collection::<Event<T>>("events")
    }

    pub async fn write_event<T: EventData>(&self, data: &T, code: TypeId) {
        println!("Writing event!");

        let kind = self
            .kinds
            .get(&code)
            .expect(&format!("No kind name recorded for event code {:?}", code))
            .to_string();

        let event = Event::<T>::new(*data, kind);
        let result = self.collection().insert_one(event, None).await;
        println!("{:?}", result);
    }
}
