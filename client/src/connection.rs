use std::{
    net::{SocketAddr, UdpSocket, Ipv4Addr},
    time::SystemTime,
};

use bevy::prelude::*;
use bevy_renet::{
    renet::{
        transport::{
            ClientAuthentication, NetcodeClientTransport, NetcodeServerTransport,
            NetcodeTransportError, ServerAuthentication, ServerConfig,
        },
        ConnectionConfig, RenetClient,
    },
    transport::NetcodeClientPlugin,
    RenetClientPlugin,
};
use lib::{connection_config, ClientChannel, ServerChannel, PROTOCOL_ID};

pub fn new_renet_client() -> (RenetClient, NetcodeClientTransport) {

    let server_channels_config = ServerChannel::channels_config();
    let client_channels_config = ClientChannel::channels_config();
    let port = 4000;

    let client = RenetClient::new(ConnectionConfig {
        server_channels_config,
        client_channels_config,
        ..Default::default()
    });

    let current_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();
    let client_id = current_time.as_millis() as u64;
    let server_addr = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), port);
    let socket = UdpSocket::bind(("127.0.0.1", 0)).unwrap();
    let authentication = ClientAuthentication::Unsecure {
        client_id,
        protocol_id: PROTOCOL_ID,
        server_addr,
        user_data: None,
    };
    let transport = NetcodeClientTransport::new(current_time, authentication, socket).unwrap();
    (client, transport)
}
