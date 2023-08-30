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
