use std::time::Duration;

use bevy_renet::renet::{ConnectionConfig, ChannelConfig, SendType};
use components::ComponentType;
use serde::{Deserialize, Serialize};
use bevy::prelude::*;

pub mod channels;
pub mod components;
pub mod resources;

#[derive(Event, Debug, Serialize, Deserialize)]
pub struct UpdateEvent {
    pub id: u64,
    pub entity: Entity,
    pub component: ComponentType,
}

