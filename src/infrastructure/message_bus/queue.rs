use bson::oid;

use std::{collections::VecDeque, sync::Arc};

use crate::infrastructure::persistence::events::EventService;

use super::{
    command_kind::CommandKind,
    event_kind::EventKind,
    message::{Message, MessageKind},
};

pub struct MessageQueue {
    queue: VecDeque<Message>,
    event_store: Arc<EventService>,
}

impl MessageQueue {
    pub fn new(event_store: Arc<EventService>) -> MessageQueue {
        MessageQueue {
            event_store,
            queue: VecDeque::new(),
        }
    }

    pub fn pop_queue(&mut self) -> Option<Message> {
        self.queue.pop_front()
    }

    pub fn send_command(&mut self, kind: CommandKind, command: bson::Bson) {
        let message = Message::new(MessageKind::Command(kind), command);
        self.send(message);
    }

    pub async fn raise_event(
        &mut self,
        correlation_id: oid::ObjectId,
        kind: EventKind,
        data: bson::Bson,
    ) -> Result<(), mongodb::error::Error> {
        self.event_store
            .write_event(correlation_id, kind.clone(), data.clone())
            .await?;

        self.send(Message::new(MessageKind::Event(kind), data));

        Ok(())
    }

    fn send(&mut self, message: Message) {
        self.queue.push_back(message);
    }
}
