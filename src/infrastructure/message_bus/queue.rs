use bson::oid;

use std::{collections::VecDeque, sync::Arc};

use crate::infrastructure::persistence::events::EventService;

use super::message::Message;

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

    // TODO: Find a solution that isn't "stringly typed".
    pub fn send_command(&mut self, kind: String, command: bson::Bson) {
        let message = Message::new(kind, command);
        self.send(message);
    }

    pub async fn raise_event(
        &mut self,
        correlation_id: oid::ObjectId,
        kind: String,
        data: bson::Bson,
    ) {
        self.event_store
            .write_event(correlation_id, kind.clone(), data.clone())
            .await;
        self.send(Message::new(kind, data));
    }

    fn send(&mut self, message: Message) {
        println!("Sending message: {}", message.kind());
        self.queue.push_back(message);
    }
}
