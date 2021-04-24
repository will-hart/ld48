use bevy::prelude::*;
use lighting::{point_lighting, LightingStatus};

use sf_core::{
    colors::{to_u8s, Colors},
    dims::Dims,
    entity::{Particle, Sink, Spawner},
    input::InputState,
    map::Map,
    GameState, StaticEntity,
};

pub mod lighting;

#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemLabel)]
enum MenuStage {
    Movement,
    Spawning,
}

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(LightingStatus {
            enabled: true,
            disable_handled: false,
        })
        .add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_system(destroy_on_click.system().before(MenuStage::Movement))
                .with_system(sink_consumption.system().before(MenuStage::Movement))
                .with_system(sand_updater.system().label(MenuStage::Movement))
                .with_system(
                    spawner_emission
                        .system()
                        .label(MenuStage::Spawning)
                        .after(MenuStage::Movement),
                )
                .with_system(point_lighting.system().after(MenuStage::Spawning)),
        )
        .add_system_set(SystemSet::on_enter(GameState::Playing).with_system(spawn_map.system()))
        .add_system_set(SystemSet::on_exit(GameState::Playing).with_system(despawner.system()));
    }
}

#[derive(Default)]
pub struct LastSpawn {
    pub time: f64,
}

pub fn spawn_map(
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
    });

    commands.spawn().insert(Sink {
        pos: (50, 0),
        sink_rate: 0.1,
        next_sink: 0.,
        sink_limit: 1000,
    });
}

pub fn sand_updater(
    time: Res<Time>,
    mut map: ResMut<Map>,
    dims: Res<Dims>,
    colours: Res<Colors>,
    mut query: Query<&mut Particle, Without<StaticEntity>>,
) {
    let empty_colour = to_u8s(colours.walls);
    let t = time.seconds_since_startup();
    let next_t = t + 1. / 60.; // update particles at 60fps

    for mut particle in query.iter_mut() {
        if particle.y() == 0 || t < particle.next_update {
            continue;
        }

        particle.next_update = next_t;
        let pos = particle.pos.clone();

        let next_pos = particle.get_next_pos();
        let mut x = next_pos.0;
        let y = next_pos.1;

        // check to see if we can move diagonally
        match map.get(x, y) {
            Some(_) => {
                if x > 0 && map.get(x - 1, y).is_none() {
                    x -= 1;
                } else if x < dims.tex_w - 1 && map.get(x + 1, y).is_none() {
                    x += 1;
                } else {
                    continue;
                }
            }
            _ => {}
        }

        particle.pos = Vec2::new(x as f32, y as f32);

        map.move_entity(
            &dims,
            (pos.x.floor() as u32, pos.y.floor() as u32),
            (x, y),
            empty_colour,
        );
    }
}

pub fn despawner(
    mut commands: Commands,
    colours: Res<Colors>,
    dims: Res<Dims>,
    mut map: ResMut<Map>,
    mut query: Query<(&Particle, &Entity)>,
) {
    for (_, &ent) in query.iter_mut() {
        commands.entity(ent).despawn();
    }

    let bg = to_u8s(colours.menu);
    map.clear(dims, &bg);
}

pub fn destroy_on_click(
    mut commands: Commands,
    input: Res<InputState>,
    mut map: ResMut<Map>,
    colours: Res<Colors>,
    dims: Res<Dims>,
    static_items: Query<&Particle, With<StaticEntity>>,
) {
    if input.mouse_down {
        let x = input.cursor_pos.x.floor() as u32;
        let y = input.cursor_pos.y.floor() as u32;

        if let Some(entity) = map.get(x, y) {
            match static_items.get(entity) {
                Ok(_) => {
                    // clear the map at this location
                    map.destroy_at(x, y, &dims, &to_u8s(colours.walls));

                    // despawn the entity
                    commands.entity(entity).despawn();
                }
                _ => {}
            }
        }
    }
}

pub fn spawner_emission(
    mut commands: Commands,
    time: Res<Time>,
    dims: Res<Dims>,
    mut map: ResMut<Map>,
    mut spawners: Query<(&mut Spawner, Entity)>,
) {
    let now = time.seconds_since_startup();
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
            next_update: 0.,
        };
        let entity = commands.spawn().insert(particle).id();

        map.spawn_entity(&dims, particle, entity);
    }
}

pub fn sink_consumption(
    mut commands: Commands,
    time: Res<Time>,
    dims: Res<Dims>,
    colours: Res<Colors>,
    mut map: ResMut<Map>,
    mut sinks: Query<(&mut Sink, Entity)>,
) {
    let now = time.seconds_since_startup();
    let clear_colour = to_u8s(colours.walls);

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
