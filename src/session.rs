pub mod endpoint;
pub mod clipboard_manager;

use endpoint::{Endpoint, ConnectionInfo, MAX_MESSAGE_BUFFER};
use clipboard_manager::ClipboardManager;
use log::{debug, info, error};
use nix::sys::select::FdSet;
use std::{str, io};

const STDIN_FD: i32 = 0;

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

    fn handle_recv_endpoint(&mut self) {
        let mut payload = [0; MAX_MESSAGE_BUFFER];
        match self.endpoint.read(&mut payload) {
            Err(e) => {
                match e.kind() {
                    io::ErrorKind::WouldBlock => { /* We are fine with nothing to read */ },
                    _ => panic!("Got an error: {}", e),
                }
            },
            Ok(payload_length) => {
                info!("Received from socket: {:?}", &payload[0..payload_length]);
                self.clipboard_manager.set_content(String::from(str::from_utf8(&payload[0..payload_length]).unwrap()));
            },
        };        
    }

    fn handle_read_stdin(&mut self) {
        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).unwrap();
        info!("User input: {:?}", user_input);
        self.clipboard_manager.set_content(user_input);
    }

    fn dispatch_read_handlers(&mut self, ready_read_fds: FdSet) {
        let endpoint_fd = self.endpoint.get_fd();

        if ready_read_fds.contains(endpoint_fd) {
            self.handle_recv_endpoint();
        }

        if ready_read_fds.contains(STDIN_FD) {
            self.handle_read_stdin();
        }
    }

    pub fn run(&mut self) {
        info!("Session run...");
        let mut read_fd_set = nix::sys::select::FdSet::new();
        let endpoint_fd = self.endpoint.get_fd();

        read_fd_set.insert(STDIN_FD); // Monitor user input
        read_fd_set.insert(endpoint_fd); // Monitor endpoint

        loop {
            let mut ready_read_fds = read_fd_set.clone();
            match nix::sys::select::select(
                Some(endpoint_fd + 1),
                &mut ready_read_fds,
                None,
                None,
                None,
            ) {
                Ok(num_fds_ready) => {
                    if num_fds_ready == 0 {
                        info!("Closing Session...");
                        break;
                    }
                    self.dispatch_read_handlers(ready_read_fds);
                },
                _ => {
                    error!("Error occurred, Closing Session...");
                    break;
                }
            }
        }
    }
}
