pub mod endpoint;
pub mod clipboard_manager;

use endpoint::{Endpoint, ConnectionInfo, Message, MAX_MESSAGE_BUFFER};
use clipboard_manager::ClipboardManager;
use log::{debug, info};
use std::str;
use futures::select;
use futures::future::FutureExt;

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

    pub async fn monitor_endpoint(&mut self) -> Message {
        let mut buf = [0; MAX_MESSAGE_BUFFER];
        let size: usize = self.endpoint.read(&mut buf).unwrap();
        Message {buf, size}
    }

    pub async fn run(&mut self) {
        info!("Session run...");
        self.endpoint.write(&[0x41, 0x41, 0x41, 0x41, 0x41]).unwrap();
        loop {
            select! {
                msg = self.monitor_endpoint().fuse() => { 
                    let received_buf = &msg.buf[0..msg.size];
                    println!("Received from socket: {:?}", received_buf);
                    self.clipboard_manager.set_content(String::from(str::from_utf8(received_buf).unwrap()));
                }
            }
        }
    }
}
