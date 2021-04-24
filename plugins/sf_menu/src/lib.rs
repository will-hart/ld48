use bevy::prelude::*;
use rand::Rng;

use sf_core::{
    colors::{to_u8s, Colors},
    dims::Dims,
    entity::WorldEntity,
    map::Map,
    GameState,
};

const SPAWN_RATE: f64 = 20.;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_update(GameState::Menu)
                .with_system(menu_sand_spawner.system())
                .with_system(sand_updater.system()),
        )
        .add_system_set(SystemSet::on_exit(GameState::Menu).with_system(despawner.system()));
    }
}

#[derive(Default)]
pub struct LastSpawn {
    pub time: f64,
}

pub fn menu_sand_spawner(
    mut commands: Commands,
    time: Res<Time>,
    colors: Res<Colors>,
    dims: Res<Dims>,
    mut map: ResMut<Map>,
    mut last_spawn: Local<LastSpawn>,
) {
    let mut rng = rand::thread_rng();

    if rng.gen_bool((last_spawn.time * SPAWN_RATE).clamp(0., 1.)) {
        last_spawn.time = 0.; // reset timer

        // spawn!
        let random_x = rng.gen_range(0..dims.tex_w);
        let world_entity = WorldEntity {
            pos: Vec2::new(random_x as f32, (dims.tex_h - 1) as f32),
            color: colors.sand,
            vel: Vec2::new(0., -1.),
            is_static: false,
        };

        map.spawn_entity(&dims, world_entity.clone());
        commands.spawn().insert(world_entity);
    }

    last_spawn.time += time.delta_seconds_f64();
}

pub fn sand_updater(mut map: ResMut<Map>, dims: Res<Dims>, mut query: Query<&mut WorldEntity>) {
    for mut particle in query.iter_mut() {
        let vel = particle.vel.clone();
        let pos = particle.pos.clone();
        particle.pos += vel;

        map.move_entity(
            &dims,
            (pos.x.floor() as u32, pos.y.floor() as u32),
            (particle.pos.x.floor() as u32, particle.pos.y.floor() as u32),
        );
    }
}

pub fn despawner(
    mut commands: Commands,
    colours: Res<Colors>,
    dims: Res<Dims>,
    mut map: ResMut<Map>,
    mut query: Query<(&WorldEntity, &Entity)>,
) {
    for (_, &ent) in query.iter_mut() {
        commands.entity(ent).despawn();
    }

    let bg = to_u8s(colours.menu);
    map.clear(dims, &bg);
}
