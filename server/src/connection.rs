use std::{
    net::{Ipv4Addr, SocketAddr, UdpSocket},
    time::SystemTime,
};

use bevy_renet::renet::{
    transport::{NetcodeServerTransport, ServerAuthentication, ServerConfig},
    ConnectionConfig, RenetServer,
};
use lib::{connection_config, ClientChannel, ServerChannel, PROTOCOL_ID};

pub fn new_renet_server() -> (RenetServer, NetcodeServerTransport) {
    let server_channels_config = ServerChannel::channels_config();
    let client_channels_config = ClientChannel::channels_config();
    let port = 4000;

    let server = RenetServer::new(ConnectionConfig {
        server_channels_config,
        client_channels_config,
        ..Default::default()
    });

    let current_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();
    let public_addr = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), port);
    let socket = UdpSocket::bind(public_addr).unwrap();
    let server_config = ServerConfig {
        max_clients: 5,
        protocol_id: PROTOCOL_ID,
        public_addr,
        authentication: ServerAuthentication::Unsecure,
    };
    let transport = NetcodeServerTransport::new(current_time, server_config, socket).unwrap();
    (server, transport)
}
