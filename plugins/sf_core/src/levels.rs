use crate::{
    colors::Colors,
    dims::Dims,
    entity::{Particle, ParticleType, Spawner},
    map::Map,
    GameState, Player, Position, StaticEntity,
};
use bevy::prelude::*;
use std::ops::Range;

pub struct NextLevel(pub u32);

pub struct Wall {
    pub points: Vec<(u32, u32)>,
}

impl Wall {
    pub fn from_x_range(x_range: Range<u32>, y: u32) -> Self {
        Wall {
            points: x_range.map(|x| (x, y)).collect::<Vec<_>>(),
        }
    }
    pub fn from_y_range(y_range: Range<u32>, x: u32) -> Self {
        Wall {
            points: y_range.map(|y| (x, y)).collect::<Vec<_>>(),
        }
    }
}

pub struct Level {
    pub player_spawn: (u32, u32),
    pub player_slime_target: u32,
    pub walls: Vec<Wall>,
    pub spawners: Vec<Spawner>,
    pub sinks: Vec<Spawner>,
}

impl Level {
    pub fn level_one(colours: &Res<Colors>) -> Self {
        Level {
            player_spawn: (5, 93),
            player_slime_target: 40,
            walls: vec![
                Wall::from_x_range(61..63, 93),
                Wall::from_x_range(50..75, 50),
                Wall::from_x_range(0..15, 90),
                Wall::from_y_range(50..53, 50),
                Wall::from_y_range(50..53, 75),
            ],
            spawners: vec![Spawner {
                pos: (62, 95),
                spawn_limit: 40,
                spawn_delay: 0.25,
                initial_vel: Vec2::new(0., -1.),
                color: colours.sand.clone(),
                next_spawn: 0.,
                particle_type: ParticleType::Sand,
            }],
            sinks: vec![],
        }
    }
}

pub fn spawn_level(
    mut commands: Commands,
    dims: Res<Dims>,
    colours: Res<Colors>,
    mut map: ResMut<Map>,
    mut state: ResMut<State<GameState>>,
    mut next_level: Query<(&NextLevel, Entity)>,
    mut players: Query<(&mut Player, &mut Position, &mut Transform)>,
) {
    for (_, entity) in next_level.iter_mut() {
        let level = Level::level_one(&colours);

        // move the player to the right spawn pos and configure them
        for (mut player, mut pos, mut tx) in players.iter_mut() {
            pos.0 = level.player_spawn.0;
            pos.1 = level.player_spawn.1;

            player.slime_target = level.player_slime_target;

            tx.translation = dims
                .grid_to_world(
                    level.player_spawn.0,
                    level.player_spawn.1,
                    Vec2::new(0., 24.),
                )
                .extend(0.);
        }

        // spawn walls
        for wall in level.walls {
            for (x, y) in wall.points {
                let particle = Particle {
                    pos: Vec2::new(x as f32, y as f32),
                    vel: Vec2::ZERO,
                    color: colours.menu.clone(),
                    particle_type: ParticleType::Obstacle,
                    next_update: f64::MAX,
                };

                let entity = commands.spawn().insert(particle).insert(StaticEntity).id();
                map.spawn_entity(&dims, particle, entity);
            }
        }

        // create spawners
        for spawner in level.spawners {
            commands.spawn().insert(spawner);
        }

        // remove the next level marker
        commands.entity(entity).despawn();

        state.set(GameState::Playing).unwrap();
        println!("Level spawned");
        break;
    }
}