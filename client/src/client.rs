use bevy::prelude::*;
use bevy_renet::{
    transport::NetcodeClientPlugin,
    RenetClientPlugin, renet::{RenetClient, DefaultChannel},
};
use connection::new_renet_client;
use lib::ServerChannel;
pub mod connection;
fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.add_plugins(RenetClientPlugin);
    app.add_plugins(NetcodeClientPlugin);
    let (client, transport) = new_renet_client(); 
    app.insert_resource(client); 
    app.insert_resource(transport);
    app.add_systems(Update, receive_server_message);
    app.run();
}

fn receive_server_message(mut client: ResMut<RenetClient>){
 while let Some(message) = client.receive_message(ServerChannel::ServerMessages) {
        println!("{:?}", message);
    }

}
