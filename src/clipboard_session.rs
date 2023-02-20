use crate::endpoint::{Endpoint};
use log::{debug, info};

pub struct ClipboardSession {
    endpoint: Endpoint,
}

impl ClipboardSession {
    pub fn new(endpoint: Endpoint) -> Self {
        debug!("Creating ClipboardSession...");
        return ClipboardSession {
            endpoint
        }
    }

    pub fn setup(&mut self) {
        debug!("ClipboardSession Setup...");
        self.endpoint.setup();
    }

    pub fn teardown(&mut self) {
        debug!("ClipboardSession Teardown...");
        self.endpoint.teardown();
    }

    pub fn run(&mut self) {
        info!("ClipboardSession run...")
    }
}
