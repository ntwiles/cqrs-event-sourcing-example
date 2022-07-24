use bson::{doc, oid};
use chrono::{offset::Utc, DateTime};
use dotenv_codegen::dotenv;
use mongodb::{options::ClientOptions, Client};
use serde::Serialize;

use std::fmt::Debug;

use crate::infrastructure::message_bus::event::EventData;

#[derive(Debug, Serialize)]
pub struct Event<T: EventData> {
    data: T,
    kind: String,
    correlation_id: oid::ObjectId,
    when: DateTime<Utc>,
}

impl<T: EventData> Event<T> {
    pub fn new<U: EventData>(correlation_id: oid::ObjectId, data: U) -> Event<U> {
        Event {
            data,
            kind: data.kind(),
            correlation_id,
            when: Utc::now(),
        }
    }
}

pub struct EventsService {
    db: mongodb::Database,
}

impl EventsService {
    pub async fn new() -> EventsService {
        let client_options = ClientOptions::parse(dotenv!("MONGODB_CONNECTION_STRING"))
            .await
            .unwrap();

        let db = Client::with_options(client_options)
            .unwrap()
            .database("cqrs-event-sourcing");

        EventsService { db }
    }

    fn collection<T: EventData>(&self) -> mongodb::Collection<Event<T>> {
        self.db.collection::<Event<T>>("events")
    }

    pub async fn write_event<T: EventData>(&self, correlation_id: oid::ObjectId, data: T) {
        println!("Writing event!");

        let event = Event::<T>::new(correlation_id, data);
        let result = self.collection().insert_one(event, None).await;
        println!("{:?}", result);
    }

    pub async fn find_events<T: EventData>(
        &self,
        correlation_id: oid::ObjectId,
        kind: String,
    ) -> Result<mongodb::Cursor<Event<T>>, mongodb::error::Error> {
        self.collection::<T>()
            .find(
                doc! {
                    "correlation_id": correlation_id,
                    "kind": kind
                },
                None,
            )
            .await
    }
}
