mod cmd_config;
mod endpoint;

use endpoint::{Endpoint};
use std::net::{Ipv4Addr};
use std::net::{TcpListener, TcpStream};
use cmd_config::{Action, Args};
use clap::Parser;

fn connect_server() -> Endpoint {
    let listener = TcpListener::bind((Ipv4Addr::LOCALHOST, 0)).unwrap();
    let bound_addr = listener.local_addr().unwrap();
    println!("Listening on {:?}...", bound_addr);
    println!("On peers machine run: clipbond connect {:?} {:?}", bound_addr.ip(), bound_addr.port());
    let (stream, addr) = listener.accept().expect("No client found!");
    println!("Accepted connection from: {:?}", addr);
    Endpoint::new(stream)
}

fn connect_client(server_ip: Ipv4Addr, server_port: u16) -> Endpoint {
    println!("Running Client (connect to: {server_ip}:{server_port})...");
    let stream = TcpStream::connect((server_ip, server_port)).expect("Couldn't connect to server");
    println!("Connected!");
    Endpoint::new(stream)
}

fn main() {
    let args = Args::parse();
    let endpoint = match args.action {
        Action::Server => {connect_server()},
        Action::Connect {
            server_ip,
            server_port
        } => {connect_client(server_ip, server_port)},
    };
}
