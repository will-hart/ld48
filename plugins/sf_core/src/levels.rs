use crate::{
    colors::{to_u8s, Colors},
    dims::Dims,
    entity::{Particle, ParticleType, Sink, Spawner},
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
    pub fn from_y_range(x: u32, y_range: Range<u32>) -> Self {
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
    pub sinks: Vec<Sink>,
    pub message: String,

    pub starting_light: u32,
    pub max_light: u32,
    pub light_decay: f64,
}

impl Level {
    pub fn get_level(colours: &Res<Colors>, level: u32) -> Option<Self> {
        println!("Generating level {}", level);
        if level == 1 {
            Some(Level::level_one(colours))
        } else if level == 2 {
            Some(Level::level_two(colours))
        // } else if level == 3 {
        // Some(Level::level_three(colours))
        } else {
            None
        }
    }

    pub fn level_one(colours: &Res<Colors>) -> Self {
        Level {
            player_spawn: (5, 93),
            player_slime_target: 40,
            starting_light: 170,
            max_light: 170,
            light_decay: 0.5,
            message: "Collect enough slime to exit at the bottom of the level...".into(),
            walls: vec![
                // starting
                Wall::from_x_range(0..15, 90),
                // steps down
                Wall::from_x_range(20..35, 70),
                Wall::from_x_range(40..45, 60),
                // slime catcher
                Wall::from_x_range(57..65, 50),
                Wall::from_y_range(55, 50..54),
                Wall::from_y_range(65, 50..54),
                // slime catcher below
                Wall::from_x_range(50..70, 30),
                Wall::from_y_range(50, 30..33),
                Wall::from_y_range(70, 30..33),
            ],
            spawners: vec![
                Spawner {
                    pos: (62, 95),
                    spawn_limit: 500,
                    spawn_delay: 0.01,
                    initial_vel: Vec2::new(0., -1.),
                    color: colours.sand.clone(),
                    next_spawn: 0.,
                    particle_type: ParticleType::Sand,
                },
                Spawner {
                    pos: (61, 95),
                    spawn_limit: 1000,
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
            starting_light: 30,
            max_light: 50,
            light_decay: 1.1,
            message: "Slime powers your light, finish before your it gets dark".into(),
            walls: vec![
                // starting
                Wall::from_x_range(0..15, 3),
                // bottom bucket
                Wall::from_x_range(55..65, 0),
                Wall::from_y_range(55, 0..23),
                Wall::from_y_range(65, 0..5),
                // steps up
                Wall::from_x_range(10..25, 12),
                Wall::from_x_range(30..39, 20),
                Wall::from_y_range(39, 20..23),
                Wall::from_x_range(39..60, 23),
                Wall::from_y_range(60, 23..40),
                // switch back stairs with some small pools
                Wall::from_x_range(27..39, 32),
                // first bucket
                Wall::from_x_range(18..22, 38),
                Wall::from_y_range(18, 38..42),
                Wall::from_y_range(22, 38..42),
                // second bucket
                Wall::from_x_range(10..15, 45),
                Wall::from_y_range(10, 45..49),
                Wall::from_y_range(15, 45..47),
                // third bucket
                Wall::from_x_range(20..26, 52),
                Wall::from_y_range(20, 52..54),
                Wall::from_y_range(26, 52..56),
                // walkway from third bucket with drop
                Wall::from_x_range(30..42, 60),
                Wall::from_y_range(42, 60..65),
                Wall::from_x_range(47..60, 58),
                //walkway from bottom bucket to right
                Wall::from_x_range(65..70, 7),
                Wall::from_x_range(73..78, 15),
                Wall::from_x_range(82..89, 23),
                Wall::from_y_range(89, 15..28),
                Wall::from_x_range(89..105, 17),
            ],
            spawners: vec![
                Spawner {
                    pos: (62, 95),
                    spawn_limit: 500,
                    spawn_delay: 0.5,
                    initial_vel: Vec2::new(0., -1.),
                    color: colours.blue_sand.clone(),
                    next_spawn: 0.,
                    particle_type: ParticleType::Liquid,
                },
                Spawner {
                    pos: (62, 95),
                    spawn_limit: 50,
                    spawn_delay: 0.01,
                    initial_vel: Vec2::new(0., -1.),
                    color: colours.blue_sand.clone(),
                    next_spawn: 0.,
                    particle_type: ParticleType::Liquid,
                },
                Spawner {
                    pos: (15, 21),
                    spawn_limit: 3,
                    spawn_delay: 0.01,
                    initial_vel: Vec2::new(0., -1.),
                    color: colours.sand.clone(),
                    next_spawn: 0.,
                    particle_type: ParticleType::Sand,
                },
                Spawner {
                    pos: (20, 46),
                    spawn_limit: 9,
                    spawn_delay: 0.01,
                    initial_vel: Vec2::new(0., -1.),
                    color: colours.sand.clone(),
                    next_spawn: 0.,
                    particle_type: ParticleType::Sand,
                },
                Spawner {
                    pos: (12, 52),
                    spawn_limit: 9,
                    spawn_delay: 0.01,
                    initial_vel: Vec2::new(0., -1.),
                    color: colours.red_sand.clone(),
                    next_spawn: 0.,
                    particle_type: ParticleType::Sand,
                },
                Spawner {
                    pos: (24, 58),
                    spawn_limit: 9,
                    spawn_delay: 0.01,
                    initial_vel: Vec2::new(0., -1.),
                    color: colours.sand.clone(),
                    next_spawn: 0.,
                    particle_type: ParticleType::Sand,
                },
                Spawner {
                    pos: (92, 30),
                    spawn_limit: 200,
                    spawn_delay: 0.3,
                    initial_vel: Vec2::new(0., -1.),
                    color: colours.red_sand.clone(),
                    next_spawn: 0.,
                    particle_type: ParticleType::Sand,
                },
            ],
            sinks: vec![
                Sink {
                    pos: (66, 0),
                    sink_rate: 0.5,
                    next_sink: 0.,
                    sink_limit: u32::MAX,
                },
                Sink {
                    pos: (54, 0),
                    sink_rate: 0.5,
                    next_sink: 0.,
                    sink_limit: u32::MAX,
                },
            ],
        }
    }

    pub fn level_three(colours: &Res<Colors>) -> Self {
        Level {
            player_spawn: (5, 5),
            player_slime_target: 100,
            starting_light: 150,
            max_light: 150,
            light_decay: 1.,
            message: "See how deep you can travel".into(),
            walls: vec![
                // starting
                Wall::from_x_range(0..15, 3),
                // steps up
                Wall::from_x_range(10..25, 12),
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

    match level {
        Some(level) => {
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

            // create sinks
            for sink in level.sinks {
                commands.spawn().insert(sink);
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
        None => {
            state.set(GameState::Victory).unwrap();
            println!("Victory! Player defeated the game");
        }
    }
}
