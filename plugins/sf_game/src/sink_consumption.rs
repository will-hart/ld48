use bevy::prelude::*;
use sf_core::{
    colors::{to_u8s, Colors},
    dims::Dims,
    entity::Sink,
    map::Map,
};

pub fn sink_consumption(
    mut commands: Commands,
    time: Res<Time>,
    dims: Res<Dims>,
    colours: Res<Colors>,
    mut map: ResMut<Map>,
    mut sinks: Query<(&mut Sink, Entity)>,
) {
    let now = time.seconds_since_startup();
    let clear_colour = to_u8s(colours.background);

    for (mut sink, ent) in sinks.iter_mut() {
        if sink.sink_limit == 0 {
            println!("Sinker depleted");
            commands.entity(ent).despawn();
            continue;
        }

        if now < sink.next_sink {
            continue;
        }

        if let Some(ent) = map.get(sink.pos.0, sink.pos.1) {
            sink.sink_limit -= 1;
            sink.next_sink = now + sink.sink_rate;

            map.destroy_at(sink.pos.0, sink.pos.1, &dims, &clear_colour);
            commands.entity(ent).despawn();
        }
    }
}
