use async_trait::async_trait;

use super::message::Message;

#[async_trait]
pub trait MessageHandler: Send + Sync {
    // TODO: Find a solution that isn't "stringly typed".
    fn message_kind(&self) -> String;
    async fn handle(&self, message: &Message);
}
