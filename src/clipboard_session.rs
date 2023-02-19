use crate::endpoint::{Endpoint, self};

pub struct ClipboardSession {
    endpoint: Endpoint,
}

impl ClipboardSession {
    pub fn new(endpoint: Endpoint) -> Self {
        return ClipboardSession {
            endpoint
        }
    }

    pub fn setup(&mut self) {
        self.endpoint.setup();
    }
}
