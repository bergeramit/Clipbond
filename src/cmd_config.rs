use clap::Parser;
use std::net::Ipv4Addr;

#[derive(Parser)]
#[command(author="Amit Berger & Guy Berger", version, about)]
pub struct Args {
    #[command(subcommand)]
    pub action: Action
}

#[derive(clap::Subcommand)]
pub enum Action {
    Connect {
        #[arg(name="SERVER_IP", help="Your Server's IP ( this is your peer's ip)")]
        server_ip: Ipv4Addr,

        #[arg(name="SERVER_PORT", help="The port your peer is listening on for the connection ( this is your peer's listen port)")]
        server_port: u16,
    },
    Server
}
