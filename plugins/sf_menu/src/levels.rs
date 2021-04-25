use bevy::prelude::*;
use sf_core::{
    colors::Colors,
    dims::Dims,
    entity::{Particle, ParticleType, Spawner},
    map::Map,
    StaticEntity,
};

pub fn spawn_level_one(
    mut commands: Commands,
    dims: Res<Dims>,
    mut map: ResMut<Map>,
    colours: Res<Colors>,
) {
    for x in 50..75 {
        let particle = Particle {
            pos: Vec2::new(x as f32, 50.),
            vel: Vec2::ZERO,
            color: colours.menu.clone(),
            particle_type: ParticleType::Obstacle,
            next_update: f64::MAX,
        };
        let entity = commands.spawn().insert(particle).insert(StaticEntity).id();

        map.spawn_entity(&dims, particle, entity);
    }

    for x in 71..79 {
        let particle = Particle {
            pos: Vec2::new(x as f32, 75.),
            vel: Vec2::ZERO,
            color: colours.menu.clone(),
            particle_type: ParticleType::Obstacle,
            next_update: f64::MAX,
        };
        let entity = commands.spawn().insert(particle).insert(StaticEntity).id();

        map.spawn_entity(&dims, particle, entity);
    }

    for x in 115..130 {
        let particle = Particle {
            pos: Vec2::new(x as f32, 80.),
            vel: Vec2::ZERO,
            color: colours.menu.clone(),
            particle_type: ParticleType::Obstacle,
            next_update: f64::MAX,
        };
        let entity = commands.spawn().insert(particle).insert(StaticEntity).id();

        map.spawn_entity(&dims, particle, entity);
    }

    commands.spawn().insert(Spawner {
        pos: (60, 90),
        spawn_limit: 1000,
        initial_vel: Vec2::new(0., -1.),
        color: colours.sand,
        spawn_delay: 0.05,
        next_spawn: 0.,
        particle_type: ParticleType::Sand,
    });
}
