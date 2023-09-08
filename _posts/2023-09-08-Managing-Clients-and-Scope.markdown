# client_hanlder function

So on the server we just gotta do some ECS stuff for when a client connects/disconnects,
we are just spawning a Player and a client, the Client stores some information that we'll need soon, we also add the client to the server_lobby and unfortunatley do need to make use of .clone() 
thanks to the classic borrow after move:

{% highlight rust %}
pub fn client_handler(
    mut server_lobby: ResMut<ServerLobby>,
    mut commands: Commands,
    mut events: EventReader<ServerEvent>,
    mut server: ResMut<RenetServer>,
) {
    for event in events.iter(){
        match event {
            ServerEvent::ClientConnected { client_id } => {
                println!("client connected {}", client_id);
                let player = commands.spawn(Player { id: *client_id }).id();


                let new_client = Client {
                    id: *client_id,
                    scope: Scope::get(Tile { cell: (0, 0, 0) }),
                    scoped_entities: HashSet::new(),
                    controlled_entity: player,
                };

                server_lobby.clients.insert(*client_id, new_client.clone());
                commands.spawn(new_client);
                let message = ServerMessages::PlayerConnected { id: *client_id };
                let message = bincode::serialize(&message).unwrap();
                server.broadcast_message(ServerChannel::ServerMessages, message);
                let message = SpawnEvent {
                    entity: player,
                    entity_type: EntityType::Player(Player { id: *client_id }),
                    tile: Tile { cell: (0, 0, 0) },
                };
                let message = bincode::serialize(&message).unwrap();
                server.broadcast_message(ServerChannel::Spawn, message);
            }

            ServerEvent::ClientDisconnected {client_id, reason} => {
                println!("client disconnected {} {}", client_id, reason);
                if let Some((_, client_entity)) = server_lobby.clients.remove_entry(client_id) {
                    commands
                        .entity(client_entity.controlled_entity)
                        .despawn_recursive();
                    let message = bincode::serialize(&client_id).unwrap();
                    server.broadcast_message(ServerChannel::Despawn, message);
                }
            }
        }
    }
    
}
{% endhighilight %}

# Managing scope
Now we have the clients being handled it is time to start managed the "scope" i.e which entities get synced with each client, my Scope struct has two methods, "get" which
takes a tile and returns a Scope struct, the other one is "check" which takes in &self and a tile and returns a bool based off if the tile is in Scope. All I'm using to 
calculate whether or not something is in scope is distance from the clients controlled entitiy.

{% highlight rust %}

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

{% endhighlight %}

Now the next step is adding functions that actually call scope.get() and scope.check(), one to create the initial scope, that checks for Added<Client>,
the next will check for Change<Tile>, called scope.check for every client and every single time any entity in the game moves would be a massive bottleneck, so for now I'll just be calling
scope.check() when a player moves. This will need to be updated eventually for when say an enemy monster walks into scope of the player and the player hasn't moved.

{% highlight rust %}

pub fn create_scope(
    mut clients: Query<&mut Client, Added<Client>>,
    entities: Query<(Entity, &Tile)>,
) {
    for mut client in clients.iter_mut() {
        for (e, t) in entities.iter() {
            if client.scope.check(t) && !client.scoped_entities.contains(&e) {
                client.scoped_entities.insert(e);
            }
        }
    }
}

pub fn entered_left_scope(
    mut clients: Query<&mut Client>,
    entities: Query<(Entity, &Tile, &EntityType)>,
    mut server: ResMut<RenetServer>,
    players: Query<(Entity, &Tile), (Changed<Tile>, With<Player>)>,
) {
    for mut client in clients.iter_mut() {
        for (e, t) in players.iter() {
            if client.controlled_entity == e {
                client.scope = Scope::get(*t);
                //println!("updated scope");
            }
        }
        for (entity, tile, entity_type) in entities.iter() {
            if client.scoped_entities.contains(&entity) {
                if !client.scope.check(tile) {
                    client.scoped_entities.remove(&entity);
                    //DESPAWN
                    let message = bincode::serialize(&entity).unwrap();
                    server.send_message(client.id, ServerChannel::Despawn, message)
                }
            } else if client.scope.check(tile) {
                //println!("scope spawn");
                client.scoped_entities.insert(entity);
                let message = SpawnEvent {
                    entity,
                    entity_type: *entity_type,
                    tile: *tile,
                };
                let message = bincode::serialize(&message).unwrap();
                server.send_message(client.id, ServerChannel::Spawn, message);
            }
        }
    }
}


{% end highlight %}

So thats most of the networking logic out the way, the client however still needs some logic to actually start spawning networked entites, then it's time to start creating a basic world via code,
then I'm considering creating a simple map editor with the help of bevy_inspector_egui, my idea is to create a few individual dungeon rooms then the game can randomly fit them together.
