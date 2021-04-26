use crate::{
    colors::{to_u8s, Colors},
    dims::Dims,
    entity::{Particle, ParticleType, Spawner},
    map::Map,
    GameState, LightingTarget, Player, Position, StaticEntity, TimedDespawn,
};
use bevy::prelude::*;
use rand::{thread_rng, Rng};
use std::ops::Range;

pub struct NextLevel(pub u32);
pub struct LevelMessage(pub String);

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
    pub message: String,

    pub starting_light: u32,
    pub max_light: u32,
    pub light_decay: f64,
}

impl Level {
    pub fn get_level(colours: &Res<Colors>, level: u32) -> Self {
        println!("Generating level {}", level);
        if level == 1 {
            Level::level_one(colours)
        } else if level == 2 {
            Level::level_two(colours)
        } else {
            Level::level_one(colours)
        }
    }

    pub fn level_one(colours: &Res<Colors>) -> Self {
        Level {
            player_spawn: (5, 93),
            player_slime_target: 40,
            starting_light: 170,
            max_light: 60,
            light_decay: 1.,
            message: "Collect enough slime to exit at the bottom of the level...".into(),
            walls: vec![
                // starting
                Wall::from_x_range(0..15, 90),
                // steps down
                Wall::from_x_range(20..35, 70),
                Wall::from_x_range(40..45, 60),
                // slime catcher
                Wall::from_x_range(57..65, 50),
                Wall::from_y_range(50..54, 55),
                Wall::from_y_range(50..54, 65),
                // slime catcher below
                Wall::from_x_range(50..70, 30),
                Wall::from_y_range(30..33, 50),
                Wall::from_y_range(30..33, 70),
            ],
            spawners: vec![
                Spawner {
                    pos: (62, 95),
                    spawn_limit: 250,
                    spawn_delay: 0.01,
                    initial_vel: Vec2::new(0., -1.),
                    color: colours.sand.clone(),
                    next_spawn: 0.,
                    particle_type: ParticleType::Sand,
                },
                Spawner {
                    pos: (61, 95),
                    spawn_limit: 250,
                    spawn_delay: 0.02,
                    initial_vel: Vec2::new(0., -1.),
                    color: colours.blue_sand.clone(),
                    next_spawn: 0.,
                    particle_type: ParticleType::Liquid,
                },
            ],
            sinks: vec![],
        }
    }

    pub fn level_two(colours: &Res<Colors>) -> Self {
        Level {
            player_spawn: (5, 5),
            player_slime_target: 100,
            message: "Slime powers your light, finish before your light runs out".into(),
            walls: vec![
                // starting
                Wall::from_x_range(0..150, 3),
                // steps up
            ],
            spawners: vec![Spawner {
                pos: (62, 95),
                spawn_limit: 300,
                spawn_delay: 0.5,
                initial_vel: Vec2::new(0., -1.),
                color: colours.blue_sand.clone(),
                next_spawn: 0.,
                particle_type: ParticleType::Liquid,
            }],
            sinks: vec![],
            starting_light: 35,
            max_light: 35,
            light_decay: 10.,
        }
    }
}

pub fn spawn_level(
    mut commands: Commands,
    dims: Res<Dims>,
    colours: Res<Colors>,
    mut map: ResMut<Map>,
    mut state: ResMut<State<GameState>>,
    mut next_level: ResMut<NextLevel>,
    mut players: Query<(
        &mut Player,
        &mut Position,
        &mut Transform,
        &mut LightingTarget,
    )>,
) {
    // clear the map
    let bg = to_u8s(colours.background);
    map.clear(&dims, &bg);

    // get the data for the next level
    let level = Level::get_level(&colours, next_level.0);
    let mut rng = thread_rng();

    // move the player to the right spawn pos and configure them
    for (mut player, mut pos, mut tx, mut light) in players.iter_mut() {
        pos.0 = level.player_spawn.0;
        pos.1 = level.player_spawn.1;

        tx.translation = dims
            .grid_to_world(
                level.player_spawn.0,
                level.player_spawn.1,
                Vec2::new(0., 16.),
            )
            .extend(0.);

        player.slime_target = level.player_slime_target;

        light.lighting_strength = level.starting_light;
        light.max_light_strength = level.max_light;
        light.lighting_decay_rate = level.light_decay;

        println!("Moved player to [{},{}] ({})", pos.0, pos.1, tx.translation);
    }

    // spawn walls
    for wall in level.walls {
        for (x, y) in wall.points {
            let particle = Particle {
                pos: Vec2::new(x as f32, y as f32),
                vel: Vec2::ZERO,
                color: colours.walls.clone(),
                particle_type: ParticleType::Obstacle,
                next_update: f64::MAX,
                is_left_first: rng.gen_bool(0.5),
            };

            let entity = commands.spawn().insert(particle).insert(StaticEntity).id();
            map.spawn_entity(&dims, particle, entity);
        }
    }

    // create spawners
    for spawner in level.spawners {
        commands.spawn().insert(spawner);
    }

    // increment the level
    next_level.0 += 1;

    // spawn the level message
    commands
        .spawn()
        .insert(LevelMessage(level.message))
        .insert(Timer::from_seconds(10., false))
        .insert(TimedDespawn);

    state.set(GameState::Playing).unwrap();
    println!("Level spawned");
}
