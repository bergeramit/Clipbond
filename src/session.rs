pub mod endpoint;
pub mod clipboard_manager;

use endpoint::{Endpoint, ConnectionInfo};
use clipboard_manager::ClipboardManager;
use log::{debug, info};

pub struct Session {
    endpoint: Endpoint,
    clipboard_manager: ClipboardManager
}

impl Session {
    pub fn new(connection_info: ConnectionInfo) -> Self {
        debug!("Creating Session...");
        return Session {
            endpoint: Endpoint::new(connection_info),
            clipboard_manager: ClipboardManager::new()
        }
    }

    pub fn setup(&mut self) {
        debug!("Session Setup...");
        self.endpoint.setup();
    }

    pub fn teardown(&mut self) {
        debug!("Session Teardown...");
        self.endpoint.teardown();
    }

    pub fn run(&mut self) {
        info!("Session run...")
    }
}
