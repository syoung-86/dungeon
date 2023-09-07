use bevy::prelude::*;
use bevy::utils::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Resource, Default)]
pub struct NetworkMapping {
    pub client: HashMap<Entity, Entity>,
    pub server: HashMap<Entity, Entity>,
}

impl NetworkMapping {
    pub fn add(&mut self, client_entity: &Entity, server_entity: &Entity) {
        self.client.insert(*client_entity, *server_entity);
        self.server.insert(*server_entity, *client_entity);
    }
}

