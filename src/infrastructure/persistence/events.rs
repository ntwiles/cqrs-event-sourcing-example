use bson::{doc, oid};
use chrono::{offset::Utc, DateTime};
use dotenv_codegen::dotenv;
use mongodb::{options::ClientOptions, Client};
use serde::{Deserialize, Serialize};

use std::fmt::Debug;

use crate::infrastructure::message_bus::event_kind::EventKind;

#[derive(Debug, Deserialize, Serialize)]
pub struct Event {
    data: bson::Bson,
    kind: EventKind,
    correlation_id: oid::ObjectId,
    when: DateTime<Utc>,
}

impl Event {
    pub fn new(correlation_id: oid::ObjectId, kind: EventKind, data: bson::Bson) -> Event {
        Event {
            correlation_id,
            data,
            kind,
            when: Utc::now(),
        }
    }

    pub fn data(&self) -> &bson::Bson {
        &self.data
    }

    pub fn kind(&self) -> &EventKind {
        &self.kind
    }
}

pub struct EventService {
    db: mongodb::Database,
}

impl EventService {
    pub async fn new() -> EventService {
        let client_options = ClientOptions::parse(dotenv!("MONGODB_CONNECTION_STRING"))
            .await
            .unwrap();

        let db = Client::with_options(client_options)
            .unwrap()
            .database("cqrs-event-sourcing");

        EventService { db }
    }

    fn collection(&self) -> mongodb::Collection<Event> {
        self.db.collection::<Event>("events")
    }

    pub async fn write_event(
        &self,
        correlation_id: oid::ObjectId,
        kind: EventKind,
        data: bson::Bson,
    ) {
        println!("Writing event!");

        let event = Event::new(correlation_id, kind, data);
        let result = self.collection().insert_one(event, None).await;
        println!("{:?}", result);
    }

    pub async fn find_events(
        &self,
        correlation_id: oid::ObjectId,
    ) -> Result<mongodb::Cursor<Event>, mongodb::error::Error> {
        self.collection()
            .find(
                doc! {
                    "correlation_id": correlation_id,
                },
                None,
            )
            .await
    }
}
