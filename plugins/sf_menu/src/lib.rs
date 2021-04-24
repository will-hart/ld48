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
            next_update: 0.,
        };

        map.spawn_entity(&dims, world_entity.clone());
        commands.spawn().insert(world_entity);
    }

    last_spawn.time += time.delta_seconds_f64();
}

pub fn sand_updater(
    time: Res<Time>,
    mut map: ResMut<Map>,
    dims: Res<Dims>,
    colours: Res<Colors>,
    mut query: Query<&mut WorldEntity>,
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
    mut query: Query<(&WorldEntity, &Entity)>,
) {
    for (_, &ent) in query.iter_mut() {
        commands.entity(ent).despawn();
    }

    let bg = to_u8s(colours.menu);
    map.clear(dims, &bg);
}
