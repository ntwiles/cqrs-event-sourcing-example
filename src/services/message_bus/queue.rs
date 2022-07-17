use std::sync::Arc;

use crate::services::message_bus::{event::EventData, message::MessageData};
use crate::services::persistence::event_store::EventStore;

use super::message::Message;

pub struct MessageQueue {
    queue: Vec<Message>,
    event_store: Arc<EventStore>,
}

impl MessageQueue {
    pub fn new(event_store: Arc<EventStore>) -> MessageQueue {
        MessageQueue {
            event_store,
            queue: Vec::new(),
        }
    }

    pub fn pop_queue(&mut self) -> Option<Message> {
        self.queue.pop()
    }

    pub fn send_command<T: 'static + MessageData>(&mut self, command: T) {
        let message = Message::new(command);
        self.send(message);
    }

    pub async fn raise_event<T: 'static + MessageData + EventData>(&mut self, event: T) {
        let message = Message::new(event);
        let store = self.event_store.clone();

        store.write_event(&event).await;
        self.send(message);
    }

    fn send(&mut self, message: Message) {
        println!("Sending message: {:?}", message.code());
        self.queue.push(message);
    }
}
