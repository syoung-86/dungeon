use std::{net::UdpSocket, time::SystemTime};

use bevy_renet::renet::{
    transport::{NetcodeServerTransport, ServerAuthentication, ServerConfig},
    RenetServer,
};
use lib::{connection_config, PROTOCOL_ID};

pub fn new_renet_server() -> (RenetServer, NetcodeServerTransport) {
    let server = RenetServer::new(connection_config());

    let server_addr = "127.0.0.1:5000".parse().unwrap();
    let socket = UdpSocket::bind(server_addr).unwrap();
    const MAX_CLIENTS: usize = 64;
    let server_config = ServerConfig {
        max_clients: MAX_CLIENTS,
        protocol_id: PROTOCOL_ID,
        public_addr: server_addr,
        authentication: ServerAuthentication::Unsecure,
    };
    let current_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();
    let transport = NetcodeServerTransport::new(current_time, server_config, socket).unwrap();
    (server, transport)
}
