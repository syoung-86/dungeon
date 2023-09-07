use bevy::{prelude::*, utils::HashSet};
use serde::{Deserialize, Serialize};


#[derive(Clone, Serialize, Deserialize, Component, Debug)]
pub struct Client {
    pub id: u64,
    pub scope: Scope,
    pub scoped_entities: HashSet<Entity>,
    pub controlled_entity: Entity,
}


#[derive(Clone, Copy, Serialize, Deserialize, Component, Default, Debug)]
pub struct Scope {
    pub top_left: Tile,
    pub bottom_right: Tile,
    pub up: Tile,
    pub down: Tile,
}

const SCOPE_DISTANCE: u32 = 20;
impl Scope {
    pub fn get(start: Tile) -> Scope {
        let mut scope = Scope::default();
        let mut top_left = start;
        let mut bottom_right = start;
        let mut up = start;
        let mut down = start;
        top_left.cell.0 += SCOPE_DISTANCE;
        top_left.cell.2 += SCOPE_DISTANCE;

        if bottom_right.cell.0 > SCOPE_DISTANCE {
            bottom_right.cell.0 -= SCOPE_DISTANCE;
        } else {
            bottom_right.cell.0 = 0;
        }

        if bottom_right.cell.2 > SCOPE_DISTANCE {
            bottom_right.cell.2 -= SCOPE_DISTANCE;
        } else {
            bottom_right.cell.2 = 0;
        }
        up.cell.1 += 1;
        if down.cell.1 > 0 {
            down.cell.1 -= 1;
        } else {
            down.cell.1 = 0;
        }

        scope.top_left = top_left;
        scope.bottom_right = bottom_right;

        scope.up = up;
        scope.down = down;

        scope
    }

    pub fn check(&self, pos: &Tile) -> bool {
        let x = pos.cell.0;
        let z = pos.cell.2;

        let tl_x = self.top_left.cell.0;
        let tl_z = self.top_left.cell.2;

        let br_x = self.bottom_right.cell.0;
        let br_z = self.bottom_right.cell.2;

        x <= tl_x && x >= br_x && z <= tl_z && z >= br_z
    }
}

#[derive(Debug, Serialize, Deserialize, Component)]
pub enum PlayerCommand {
    Move(Tile),
}

#[derive(bevy::prelude::Component)]
pub struct ControlledEntity;

#[derive(Eq, PartialEq, Debug, Clone, Copy, Serialize, Deserialize, Component)]
pub enum EntityType {
    Tile,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, Component)]
pub enum ComponentType {
    Tile(Tile),
}

#[derive(Event, Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize, Component)]
pub struct SpawnEvent {
    pub entity: Entity,
    pub entity_type: EntityType,
    pub tile: Tile,
}

impl SpawnEvent {
    pub fn new(entity: Entity, entity_type: EntityType, tile: Tile) -> Self {
        Self {
            entity,
            entity_type,
            tile,
        }
    }
}

#[derive(
    Reflect,
    Eq,
    PartialEq,
    Debug,
    Serialize,
    Deserialize,
    Component,
    Default,
    Copy,
    Clone,
    PartialOrd,
    Hash,
)]
#[reflect(Component)]
pub struct Tile {
    pub cell: (u32, u32, u32),
}

impl Tile {
    pub fn new(cell: (u32, u32, u32)) -> Self {
        Self { cell }
    }

    pub fn to_transform(&self) -> Transform {
        let mut transform = Vec3::new(0.0, 0.0, 0.0);
        transform[0] = self.cell.0 as f32;
        transform[1] = self.cell.1 as f32;
        transform[2] = self.cell.2 as f32;
        Transform::from_xyz(transform[0], transform[1], transform[2])
    }
    pub fn from_xyz(translation: &Vec3) -> Tile {
        let mut new_tile = Tile::default();
        new_tile.cell.0 = translation[0] as u32;
        new_tile.cell.1 = translation[1] as u32;
        new_tile.cell.2 = translation[2] as u32;
        new_tile
    }
}

#[derive(Event, Debug, Serialize, Deserialize)]
pub struct UpdateEvent {
    pub entity: Entity,
    pub component: ComponentType,
}

impl UpdateEvent {
    pub fn new(entity: Entity, component: ComponentType) -> Self {
        Self { entity, component }
    }
}

