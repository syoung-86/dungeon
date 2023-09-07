## Adding a layer of abstraction to Renet
So manualy receiving/sending packets from channels is fine but I find it much more ergonomic to employ some sort of strategy pattern that
receives any message then fires off an event and handle it with the normal ECS patterns.   

Firstly we need to map client/server entities, for saftey reasons having the same entities on client and server is a terrible idea, only the client will worry about
mapping entities, so server only has to deal with server entities. I decided to use two hashmaps, client to server and server to client, since sometimes I need to check if
one exists while I only have the other in scope.

{% highlight rust %}
//resouces.rs
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


{% endhighlight %}

Now for actually receiving packets, I have a function like this for each channel, while it may possible also abstract this into a single function that handles all the channels I feel like it would be a bit of a needless abstraction since I the amount of channels I receive with won't increase by much:

{% highlight rust %}

pub fn spawn_message(
    mut client: ResMut<RenetClient>,
    mut spawn_event: EventWriter<SpawnEvent>,
    mut network_mapping: ResMut<NetworkMapping>,
    mut commands: Commands,
) {
    if let Some(message) = client.receive_message(ServerChannel::Spawn) {
        let spawn_message: SpawnEvent = bincode::deserialize(&message).unwrap();
        if network_mapping.server.get(&spawn_message.entity).is_none() {
            let entity = commands.spawn_empty().id();
            network_mapping.add(&entity, &spawn_message.entity);
            spawn_event.send(SpawnEvent {
                entity,
                entity_type: spawn_message.entity_type,
                tile: spawn_message.tile,
            });
        }
    }
}

{% endhighlight %}

As for the server recieving commands, its pretty easy since we mainly recieve just from the one PlayerCommand channel, then we can just match out the PlayerCommand enum and fire of any event we need:

{% highlight rust %}

#[derive(Debug, Serialize, Deserialize, Component)]
pub enum PlayerCommand {
    Move(Tile),
}
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
{% endhighlight %}
