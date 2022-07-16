use async_trait::async_trait;

use std::any::TypeId;

use super::message::Message;

#[async_trait]
pub trait MessageHandler: Send + Sync {
    fn message_type(&self) -> TypeId;
    async fn handle(&self, message: &dyn Message);
}
