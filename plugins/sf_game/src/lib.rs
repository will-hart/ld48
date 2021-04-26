use bevy::prelude::*;
use lighting::{point_lighting, LightingStatus};

use sf_core::{
    colors::{to_u8s, Colors},
    dims::Dims,
    entity::{Particle, Sink, Spawner},
    map::Map,
    ui::PlayingUiElement,
    GameState, Player, StaticEntity,
};

pub mod lighting;

#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemLabel)]
enum MenuStage {
    Movement,
    Spawning,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(LightingStatus {
            enabled: true,
            disable_handled: false,
        })
        .add_system_set(
            SystemSet::on_update(GameState::Playing)
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
        .add_system_set(SystemSet::on_exit(GameState::Playing).with_system(despawner.system()));
    }
}

#[derive(Default)]
pub struct LastSpawn {
    pub time: f64,
}

pub fn sand_updater(
    time: Res<Time>,
    mut map: ResMut<Map>,
    dims: Res<Dims>,
    colours: Res<Colors>,
    mut query: Query<&mut Particle, Without<StaticEntity>>,
) {
    let empty_colour = to_u8s(colours.background);
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
    mut particles: Query<(&Particle, Entity)>,
    mut players: Query<(&Player, Entity)>,
    mut ui: Query<(&PlayingUiElement, Entity)>,
) {
    for (_, ent) in particles.iter_mut() {
        commands.entity(ent).despawn_recursive();
    }

    for (_, ent) in players.iter_mut() {
        commands.entity(ent).despawn_recursive();
    }

    for (_, ent) in ui.iter_mut() {
        commands.entity(ent).despawn_recursive();
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
            particle_type: spawner.particle_type,
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
