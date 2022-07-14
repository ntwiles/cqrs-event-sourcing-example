use dotenv_codegen::dotenv;
use mongodb::{options::ClientOptions, Client};

use crate::application::event::Event;

pub struct EventStore {}

impl EventStore {
    pub fn new() -> EventStore {
        EventStore {}
    }

    pub async fn write_event(&self, event: Event) {
        let client_options = ClientOptions::parse(dotenv!("MONGODB_CONNECTION_STRING"))
            .await
            .unwrap();

        let db = Client::with_options(client_options)
            .unwrap()
            .database("cqrs-event-sourcing");

        let collection = db.collection::<Event>("event-writes");

        let result = collection.insert_one(event, None).await;

        println!("{:?}", result);
    }
}
