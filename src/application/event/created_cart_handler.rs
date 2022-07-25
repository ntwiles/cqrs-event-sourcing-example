use async_trait::async_trait;

use crate::infrastructure::message_bus::{handler::MessageHandler, message::Message};

#[derive(Clone)]
pub struct CreatedCartEventHandler {}

impl CreatedCartEventHandler {
    pub fn new() -> CreatedCartEventHandler {
        CreatedCartEventHandler {}
    }
}

#[async_trait]
impl MessageHandler for CreatedCartEventHandler {
    fn message_kind(&self) -> String {
        "createdCart".to_string()
    }

    async fn handle(&self, _message: &Message) -> () {
        println!("Handling CreatedCartEvent!");
    }
}
