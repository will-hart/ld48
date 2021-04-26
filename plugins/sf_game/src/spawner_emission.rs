use bevy::prelude::*;
use rand::{thread_rng, Rng};
use sf_core::{
    dims::Dims,
    entity::{Particle, Spawner},
    map::Map,
};

pub fn spawner_emission(
    mut commands: Commands,
    time: Res<Time>,
    dims: Res<Dims>,
    mut map: ResMut<Map>,
    mut spawners: Query<(&mut Spawner, Entity)>,
) {
    let now = time.seconds_since_startup();
    let mut rng = thread_rng();

    for (mut spawner, ent) in spawners.iter_mut() {
        if spawner.spawn_limit == 0 {
            println!("Spawner depleted");
            commands.entity(ent).despawn();
            continue;
        }

        if now < spawner.next_spawn {
            continue;
        }

        if let Some(_) = map.get(spawner.pos.0, spawner.pos.1) {
            continue;
        }

        spawner.spawn_limit -= 1;
        spawner.next_spawn = now + spawner.spawn_delay;

        let particle = Particle {
            pos: Vec2::new(spawner.pos.0 as f32, spawner.pos.1 as f32),
            vel: spawner.initial_vel.clone(),
            color: spawner.color.clone(),
            particle_type: spawner.particle_type,
            next_update: 0.,
            is_left_first: rng.gen_bool(0.5),
        };
        let entity = commands.spawn().insert(particle).id();

        map.spawn_entity(&dims, particle, entity);
    }
}
