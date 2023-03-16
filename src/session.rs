pub mod endpoint;
pub mod clipboard_manager;

use endpoint::{Endpoint, ConnectionInfo, Message, MAX_MESSAGE_BUFFER};
use clipboard_manager::ClipboardManager;
use log::{debug, info};
use std::{str, io};

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

    pub fn handle_endpoint(&mut self) {
        let mut buf = [0; MAX_MESSAGE_BUFFER];
        //let size: usize;
        match self.endpoint.read(&mut buf) {
            Err(e) => {
                match e.kind() {
                    io::ErrorKind::WouldBlock => { },
                    _ => panic!("Got an error: {}", e),
                }
            },
            Ok(arrived_size) => {
                println!("Received from socket: {:?}", &buf[0..arrived_size]);
                self.clipboard_manager.set_content(String::from(str::from_utf8(&buf[0..arrived_size]).unwrap()));
            },
        };        
    }

    pub fn handle_stdin(&mut self) {
        let mut buf = String::new();
        io::stdin().read_line(&mut buf).unwrap();
        println!("User input: {:?}", buf);
    }

    pub fn run(&mut self) {
        info!("Session run...");
        if self.endpoint.is_client() {
            self.endpoint.write(&[0x41, 0x41, 0x41, 0x41, 0x41]).unwrap();
        }

        let mut read_fd_set = nix::sys::select::FdSet::new();

        let stdin_fd = 0;
        let endpoint_fd = self.endpoint.get_fd();
        read_fd_set.insert(stdin_fd);
        read_fd_set.insert(endpoint_fd);

        loop {
            let mut read_fds = read_fd_set.clone();
            match nix::sys::select::select(
                Some(endpoint_fd + 1),
                &mut read_fds,
                None,
                None,
                None,
            ) {
            Ok(num_fds_ready) => {
                if num_fds_ready == 0 {
                    println!("Closing Session");
                    return
                }
                if read_fds.contains(endpoint_fd) {
                    self.handle_endpoint();
                }
                if read_fds.contains(stdin_fd) {
                    self.handle_stdin();
                }
            },
            _ => {}
            }
        }
    }
}
