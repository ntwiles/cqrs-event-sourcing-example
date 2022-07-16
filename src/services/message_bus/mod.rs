pub mod event;
pub mod handler;
pub mod message;
pub mod queue;
pub mod registry;

use std::{
    sync::{Arc, Mutex},
    thread,
};

use queue::MessageQueue;
use registry::HandlerRegistry;

pub fn start_message_loop(queue: Arc<Mutex<MessageQueue>>, registry: HandlerRegistry) {
    thread::spawn(move || loop {
        let message = queue.lock().unwrap().pop_queue();

        if let Some(message) = message {
            let handlers = registry.get_handlers(message.code());

            for handler in handlers {
                handler.handle(&message);
            }
        }
    });
}
