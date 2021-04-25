use bevy::prelude::*;
use sf_core::{
    colors::{to_u8s, Colors},
    dims::Dims,
    entity::Particle,
    map::Map,
    Player,
};

/// Removes the particles around a player
pub fn player_sink(
    mut commands: Commands,
    mut map: ResMut<Map>,
    time: Res<Time>,
    dims: Res<Dims>,
    colours: Res<Colors>,
    mut players: Query<&mut Player>,
    particles: Query<(&Particle, Entity)>,
) {
    let t = time.seconds_since_startup();

    for mut player in players.iter_mut() {
        let clear_colour = to_u8s(colours.walls);

        if t < player.next_sink {
            continue;
        }

        let (x, y) = player.pos;

        for &dx in &[-1i32, 0, 1] {
            for &dy in &[-1i32, 0, 1] {
                // check lower bounds
                if (dx == -1 && x == 0) || (dy == -1 && y == 0) {
                    continue;
                }

                // check upper bounds
                if (dx == 1 && x >= dims.tex_w - 1) || (dy == 1 && y >= dims.tex_h - 1) {
                    continue;
                }

                // construct the coordinates to check
                let cx = (x as i32 + dx) as u32;
                let cy = (y as i32 + dy) as u32;

                // check if we have an entity at that point to consume
                match map.get(cx, cy) {
                    Some(ent) => match particles.get(ent) {
                        Ok((_, ent)) => {
                            // remove the slime
                            map.destroy_at(cx, cy, &dims, &clear_colour);
                            commands.entity(ent).despawn();

                            // increment the player lighting
                            player.slime_collected += 1;

                            if player.slime_collected % player.light_growth_rate == 0 {
                                player.lighting_strength = (player.lighting_strength + 1)
                                    .clamp(0, player.max_light_strength);
                            }

                            player.next_sink = t + player.sink_rate;
                        }
                        Err(_) => {}
                    },
                    None => {}
                }
            }
        }
    }
}
