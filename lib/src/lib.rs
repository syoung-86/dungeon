use serde::{Deserialize, Serialize};
use bevy::prelude::*;

#[derive(Copy, Clone, Default, Serialize, Deserialize, Eq, PartialEq, Debug, Resource)]
pub struct GameTick {
    pub tick: u64,
}
