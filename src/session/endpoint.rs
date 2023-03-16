use std::io::{Write, Read};
use std::net::{Ipv4Addr};
use std::time::Duration;
use std::os::fd::{RawFd, AsRawFd};
use log::info;
use std::net::{TcpListener, TcpStream, Shutdown};

pub const MAX_MESSAGE_BUFFER: usize = 1024;
const STREAM_TIMEOUT_MS: u64 = 100;

pub enum ConnectionInfo {
    Server {
        listening_ip: Ipv4Addr,
        listening_port: u16
    },
    Client {
        server_ip: Ipv4Addr,
        server_port: u16
    }
}

pub struct Endpoint {
    metadata: ConnectionInfo,
    stream: Option<TcpStream>,
}

impl Endpoint {
    pub fn new(metadata: ConnectionInfo) -> Self {
        Endpoint {
            metadata,
            stream: None
        }
    }

    pub fn setup(&mut self) {
        match self.metadata {
            ConnectionInfo::Server { listening_ip, listening_port } => {
                let listener = TcpListener::bind((listening_ip, listening_port)).unwrap();
                let bound_addr = listener.local_addr().unwrap();
                info!("Listening on {:?}...", bound_addr);
                info!("On peers machine run: clipbond connect {:?} {:?}", bound_addr.ip(), bound_addr.port());
                let (stream, addr) = listener.accept().expect("No client found!");
                stream.set_read_timeout(Some(Duration::from_millis(STREAM_TIMEOUT_MS))).unwrap();
                self.stream = Some(stream);
                info!("Accepted connection from: {:?}", addr);
            },
            ConnectionInfo::Client { server_ip, server_port } => {
                info!("Running Client (connect to: {server_ip}:{server_port})...");
                let stream = TcpStream::connect((server_ip, server_port)).expect("Couldn't connect to server");
                info!("Connected!");
                stream.set_read_timeout(Some(Duration::from_millis(STREAM_TIMEOUT_MS))).unwrap();
                self.stream = Some(stream);
            }
        }
    }

    pub fn teardown(&mut self) {
        self.stream.as_ref().unwrap().shutdown(Shutdown::Both).expect("shutdown call failed");
    }

    pub fn write(&mut self, buf: &[u8]) -> Result<usize, std::io::Error> {
        self.stream.as_ref().unwrap().write(buf)
    }

    pub fn read(&mut self, buf: &mut [u8]) -> Result<usize, std::io::Error> {
        self.stream.as_ref().unwrap().read(buf)
    }

    pub fn is_client(&self) -> bool {
        match self.metadata {
            ConnectionInfo::Client { .. } => { true },
            _ => { false }
        }
    }

    pub fn get_fd(&self) -> RawFd {
        self.stream.as_ref().unwrap().as_raw_fd()
    }
}
