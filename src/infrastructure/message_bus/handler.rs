use async_trait::async_trait;

use super::message::{Message, MessageKind};

#[async_trait]
pub trait MessageHandler: Send + Sync {
    fn message_kind(&self) -> MessageKind;
    async fn handle(&self, message: &Message);
}
