mod cmd_config;
mod session;

use session::endpoint::{ConnectionInfo};
use session::{Session};
use std::net::{Ipv4Addr};
use cmd_config::{Action, Args};
use clap::Parser;

fn main() {
    env_logger::init();
    let args = Args::parse();
    let connection_info = match args.action {
        Action::Server => {ConnectionInfo::Server { listening_ip: Ipv4Addr::LOCALHOST, listening_port: 0 }},
        Action::Connect {
            server_ip,
            server_port
        } => {ConnectionInfo::Client { server_ip, server_port }},
    };
    let mut session = Session::new(connection_info);
    session.setup();
    session.run();
    session.teardown();
}
