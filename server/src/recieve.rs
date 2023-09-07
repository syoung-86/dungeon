use bevy::prelude::*;
use bevy_renet::renet::RenetServer;
use lib::{channels::ClientChannel, components::PlayerCommand};

pub fn message(mut server: ResMut<RenetServer>) {
    for client_id in server.clients_id().into_iter() {
        while let Some(message) = server.receive_message(client_id, ClientChannel::Command) {
            let command = bincode::deserialize(&message).unwrap();
            match command {
                PlayerCommand::Move(_) => {
                    todo!()
                }
            }
        }
    }
}
