use super::message::Message;
use std::any::TypeId;

pub trait MessageHandler {
    fn message_type(&self) -> TypeId;
    fn handle(&self, message: &dyn Message);
}
