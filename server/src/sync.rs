use lib::{components::{Tile, Client, ComponentType}, UpdateEvent};
use bevy::prelude::*;

macro_rules! update_component {
    ($fn_name:ident, $type_name:ident) => {
        pub fn $fn_name(
            clients: Query<&Client>,
            components: Query<(Entity, &$type_name), Changed<$type_name>>,
            mut update_event: EventWriter<UpdateEvent>,
        ) {
            for client in clients.iter() {
                for (entity, component) in components.iter() {
                    if client.scoped_entities.contains(&entity) {
                        let event = UpdateEvent {
                            id: client.id,
                            entity,
                            component: ComponentType::$type_name(*component),
                        };
                        update_event.send(event);
                    }
                }
            }
        }
    };
}

update_component!(update_tile, Tile);
//dont forget to add system to App
//add to component type enum
//add macro call
//add system
//add match arm on client
