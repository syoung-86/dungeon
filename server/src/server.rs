use bevy::prelude::*;
use bevy_renet::{
    renet::{DefaultChannel, RenetServer, ServerEvent},
    transport::NetcodeServerPlugin,
    RenetServerPlugin,
};
use connection::new_renet_server;
use lib::{GameTick, ServerChannel};
use std::time::Duration;
pub mod connection;

fn main() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app.add_plugins(RenetServerPlugin);
    app.add_plugins(NetcodeServerPlugin);
    app.insert_resource(FixedTime::new(Duration::from_millis(100)));
    app.insert_resource(GameTick::default());
    let (server, transport) = new_renet_server();
    app.insert_resource(server);
    app.insert_resource(transport);
    app.add_systems(FixedUpdate, tick);
    app.add_systems(Update, handle_events_system);
    app.run();
}

fn tick(mut tick: ResMut<GameTick>) {
    tick.tick += 1;
}

fn dummy() {}
fn handle_events_system(
    mut server: ResMut<RenetServer>,
    mut server_events: EventReader<ServerEvent>,
) {
    //while let Some(event) = server.get_event() {
        //println!("event: {:?}", event);
        //server.broadcast_message(
            //ServerChannel::ServerMessages,
            //"server message".as_bytes().to_vec(),
        //);
        for event in server_events.iter() {
            match event {
                ServerEvent::ClientConnected { client_id } => {
                    println!("Client {client_id} connected");
                }
                ServerEvent::ClientDisconnected { client_id, reason } => {
                    println!("Client {client_id} disconnected: {reason}");
                }
            }
        }
    }
//}
