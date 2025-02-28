use std::any::Any;
use std::collections::HashMap;
use std::sync::{Arc, Mutex, OnceLock};

#[derive(Debug, Eq, PartialEq, Hash)]
enum MessageID {
    Electrical,
    Hydraulic,
}

struct CommunicationBus {
    messages: Mutex<HashMap<MessageID, Vec<Arc<dyn Any + Send + Sync>>>>,
}

impl CommunicationBus {
    fn new() -> Self {
        Self {
            messages: Mutex::new(HashMap::new()),
        }
    }

    fn instance() -> &'static Arc<Self> {
        static INSTANCE: OnceLock<Arc<CommunicationBus>> = OnceLock::new();
        INSTANCE.get_or_init(|| Arc::new(CommunicationBus::new()))
    }

    fn send<T: 'static + Send + Sync>(&self, id: MessageID, message: T) {
        let mut messages = self.messages.lock().unwrap();
        messages
            .entry(id)
            .or_insert_with(|| Vec::new())
            .push(Arc::new(message));
    }

    fn receive<T: 'static + Send + Sync + Clone>(&self, id: MessageID) -> Vec<T> {
        let messages = self.messages.lock().unwrap();
        if let Some(vec) = messages.get(&id) {
            vec.iter()
                .filter_map(|msg| msg.clone().downcast::<T>().ok().map(|arc| (*arc).clone()))
                .collect()
        } else {
            Vec::new()
        }
    }
}
