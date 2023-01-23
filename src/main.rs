mod cmd_config;

use std::net::Ipv4Addr;
use cmd_config::{Action, Args};
use clap::Parser;

fn run_server() {
    println!("Running Server...")
}

fn run_client(server_ip: Ipv4Addr, server_port: u16) {
    println!("Running Client (connect to: {server_ip}:{server_port})...")
}

fn main() {
    let args = Args::parse();
    match args.action {
        Action::Server => {
            run_server();
        },
        Action::Connect {server_ip, server_port} => {
            run_client(server_ip, server_port)
        },
    };
}
