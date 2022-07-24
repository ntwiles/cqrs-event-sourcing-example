use bson::oid;

use std::{collections::VecDeque, sync::Arc};

use crate::infrastructure::message_bus::{event::EventData, message::MessageData};
use crate::infrastructure::persistence::events::EventsService;

use super::message::Message;

pub struct MessageQueue {
    queue: VecDeque<Message>,
    event_store: Arc<EventsService>,
}

impl MessageQueue {
    pub fn new(event_store: Arc<EventsService>) -> MessageQueue {
        MessageQueue {
            event_store,
            queue: VecDeque::new(),
        }
    }

    pub fn pop_queue(&mut self) -> Option<Message> {
        self.queue.pop_front()
    }

    pub fn send_command<T: 'static + MessageData>(&mut self, command: T) {
        let message = Message::new(command);
        self.send(message);
    }

    pub async fn raise_event<T: 'static + MessageData + EventData>(
        &mut self,
        correlation_id: oid::ObjectId,
        event: T,
    ) {
        self.event_store.write_event(correlation_id, event).await;
        self.send(Message::new(event));
    }

    fn send(&mut self, message: Message) {
        println!("Sending message: {:?}", message.code());
        self.queue.push_back(message);
    }
}
