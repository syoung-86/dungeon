use bevy::prelude::*;
use bevy_renet::{renet::RenetClient, transport::NetcodeClientPlugin, RenetClientPlugin};
use camera::{setup_camera, camera_follow};
use connection::new_renet_client;
use lib::{channels::ServerChannel, components::SpawnEvent, resources::GameTick};
use receive::{spawn_message, tick};
use resources::NetworkMapping;
use smooth_bevy_cameras::controllers::orbit::OrbitCameraPlugin;
use bevy_mod_picking::prelude::*;

pub mod connection;
pub mod camera;
pub mod resources;
pub mod receive;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.add_plugins(RenetClientPlugin);
    app.add_plugins(NetcodeClientPlugin);
    app.add_plugins(
        DefaultPickingPlugins
            .build()
            .disable::<DebugPickingPlugin>(),
    );
    app.add_plugins(OrbitCameraPlugin::default());
    let (client, transport) = new_renet_client();
    app.insert_resource(client);
    app.insert_resource(transport);

    app.insert_resource(NetworkMapping::default());
    app.insert_resource(GameTick::default());

    app.add_event::<SpawnEvent>();

    app.add_systems(Update, spawn_message);
    app.add_systems(Update, tick);
    app.add_systems(Startup, setup_camera);
    app.add_systems(Update, camera_follow);
    app.run();
}

