# Syncing Components  

So syncing components between the clients/server is kinda tricky, there is pretty much two options: Reflection + dynamic dispatch or macros, I opted to use macros, 
since macros are cool, simpler, and faster. It does add a slight amount of boiler plate, but with reflection I'd need to use a few extra derive macros on every single one of my
components anyway so I'm not sold that the extra boiler plate is that big of a deal.  

So here is my basic macro:

{% highlight rust %}

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
                            entity,
                            component: ComponentType::$type_name(*component),
                        };
                        update_event.send(UpdateEvent(client.id, event));
                    }
                }
            }
        }
    };
}
{% endhighlight %} 

The "boilerplate" that is needed to use this macro goes as follows:  
* Add each synced component to the ComponentType enum
* Call the macro, e.g: update_component!(update_tile, Tile);
* add the system to the app
* have the client insert the component on the client:
{% highlight rust %}
pub fn update(
    mut commands: Commands,
    mut update_event: EventReader<UpdateEvent>,
) {
    for event in update_event.iter() {
        //println!("Received Update Event");
            match event.component {
              ComponentType::Tile(t) => {
                commands.entity(event.entity).insert(t)
              }
            }
    }

{% endhighlight %}

So there is quite a lot more that potentially needs to be done when recieving a update component message, for example converting the tile to a transform, then also
comparing the previous location to the new one to update the direction the player should be facing. But thats all pretty straightfoward really.  

Oh and this: client.scoped_entities.contains(&entity) is just a hashset of all the entities in scope of a client, this way we have complete control over each entity
that gets synced with a specific client, for example simply checking the distance to each entity from the clients controlled entity
