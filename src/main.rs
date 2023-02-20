mod cmd_config;
mod endpoint;
mod clipboard_session;

use endpoint::{Endpoint, ConnectionInfo};
use clipboard_session::ClipboardSession;
use std::net::{Ipv4Addr};
use cmd_config::{Action, Args};
use clap::Parser;

fn main() {
    env_logger::init();
    let args = Args::parse();
    let endpoint = match args.action {
        Action::Server => {Endpoint::new(ConnectionInfo::Server { listening_ip: Ipv4Addr::LOCALHOST, listening_port: 0 })},
        Action::Connect {
            server_ip,
            server_port
        } => {Endpoint::new(ConnectionInfo::Client { server_ip, server_port })},
    };
    let mut session = ClipboardSession::new(endpoint);
    session.setup();
    session.run();
    session.teardown();
}
