use bevy::prelude::Resource;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Default, Serialize, Deserialize, Eq, PartialEq, Debug, Resource)]
pub struct GameTick {
    pub tick: u64,
}
