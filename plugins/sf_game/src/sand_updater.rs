use bevy::prelude::*;
use sf_core::{
    colors::{to_u8s, Colors},
    dims::Dims,
    entity::Particle,
    map::Map,
    StaticEntity,
};

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
