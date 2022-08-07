use bson::{doc, oid};

use dotenv_codegen::dotenv;
use mongodb::{options::ClientOptions, Client, Cursor};

use crate::domain::product::Product;

pub struct ProductService {
    db: mongodb::Database,
}

impl ProductService {
    pub async fn new() -> Self {
        let client_options = ClientOptions::parse(dotenv!("MONGODB_CONNECTION_STRING"))
            .await
            .unwrap();

        let db = Client::with_options(client_options)
            .unwrap()
            .database("cqrs-event-sourcing");

        Self { db }
    }

    fn collection(&self) -> mongodb::Collection<Product> {
        self.db.collection::<Product>("products")
    }

    pub async fn find(&self) -> Result<Cursor<Product>, mongodb::error::Error> {
        self.collection().find(doc! {}, None).await
    }

    pub async fn find_by_id(
        &self,
        product_id: oid::ObjectId,
    ) -> Result<Option<Product>, mongodb::error::Error> {
        self.collection()
            .find_one(
                doc! {
                    "_id": product_id,
                },
                None,
            )
            .await
    }
}
