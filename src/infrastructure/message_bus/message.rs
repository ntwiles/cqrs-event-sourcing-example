use std::fmt::Debug;

use super::{command_kind::CommandKind, event_kind::EventKind};

#[derive(Clone, Debug, PartialEq)]
pub enum MessageKind {
    Command(CommandKind),
    Event(EventKind),
}

#[derive(Debug)]
pub struct Message {
    kind: MessageKind,
    data: bson::Bson,
}

impl Message {
    pub fn new(kind: MessageKind, data: bson::Bson) -> Message {
        Message { kind, data }
    }

    pub fn kind(&self) -> MessageKind {
        self.kind.clone()
    }

    pub fn data(&self) -> &bson::Bson {
        &self.data
    }
}
