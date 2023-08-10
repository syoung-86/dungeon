use bevy::prelude::*;
use bevy_renet::{renet::RenetServer, RenetServerPlugin};
use connection::new_renet_server;
use lib::GameTick;
use std::time::Duration;
pub mod connection;

fn main() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app.add_plugins(RenetServerPlugin);
    app.insert_resource(FixedTime::new(Duration::from_millis(100)));
    app.insert_resource(GameTick::default());
    let (server, transport) = new_renet_server();
    app.insert_resource(server);
    app.insert_resource(transport);
    app.add_systems(FixedUpdate, tick);
    app.run();
}

fn tick(mut tick: ResMut<GameTick>) {
    tick.tick += 1;
}
