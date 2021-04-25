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
    dims: Res<Dims>,
    colours: Res<Colors>,
    mut players: Query<&mut Player>,
    particles: Query<(&Particle, Entity)>,
) {
    for mut player in players.iter_mut() {
        let clear_colour = to_u8s(colours.walls);

        for dx in (player.pos.0 as i32 - 1)..(player.pos.0 as i32 + 1) {
            for dy in player.pos.1 as i32 - 1..player.pos.1 as i32 + 1 {
                if dx < 0 || dy < 0 {
                    continue;
                }

                match map.get(dx as u32, dy as u32) {
                    Some(ent) => match particles.get(ent) {
                        Ok((_, ent)) => {
                            // remove the slime
                            map.destroy_at(dx as u32, dy as u32, &dims, &clear_colour);
                            commands.entity(ent).despawn();

                            // increment the player lighting
                            player.slime_collected += 1;

                            if player.slime_collected % 10 == 0 {
                                player.lighting_strength += 1;
                            }
                        }
                        Err(_) => {}
                    },
                    None => {}
                }
            }
        }
    }
}
