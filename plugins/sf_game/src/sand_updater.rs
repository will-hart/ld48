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
        let mut y = next_pos.1;

        // if the square directly below is occupied, try some other directions
        match map.get(x, y) {
            Some(_) => {
                let neighbours = map.test_free_neighbours(x, y + 1);

                let diags = if particle.is_left_first {
                    [6, 8]
                } else {
                    [8, 6]
                };

                // randomly pick a diagonal
                if neighbours[diags[0]] {
                    if diags[0] == 6 {
                        x -= 1
                    } else {
                        x += 1
                    }
                } else if neighbours[diags[1]] {
                    if diags[1] == 6 {
                        x -= 1
                    } else {
                        x += 1
                    }
                } else {
                    match particle.particle_type {
                        sf_core::entity::ParticleType::Liquid => {
                            // don't smash about if there aren't any neighbours
                            if !neighbours[3] && !neighbours[5] {
                                continue;
                            }

                            // randomly pick a horizontal
                            let sides = if particle.is_left_first {
                                [3, 5]
                            } else {
                                [5, 3]
                            };

                            if neighbours[sides[0]] {
                                if sides[0] == 3 {
                                    x -= 1;
                                    y += 1;
                                } else {
                                    x += 1;
                                    y += 1;
                                }
                            } else if neighbours[sides[1]] {
                                if sides[1] == 3 {
                                    x -= 1;
                                    y += 1;
                                } else {
                                    x += 1;
                                    y += 1;
                                }
                            } else {
                                continue;
                            }
                        }
                        _ => {
                            continue;
                        }
                    }
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
