use bevy::prelude::*;
use lib::GameTick;
use std::time::Duration;

fn main(){
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app.insert_resource(FixedTime::new(Duration::from_millis(100)));
    app.insert_resource(GameTick::default());
    app.add_systems(FixedUpdate, tick);
    app.run();
}

fn tick(mut tick: ResMut<GameTick>) {
    tick.tick += 1;
}
