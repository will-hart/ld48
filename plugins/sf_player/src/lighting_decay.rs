// handles periodically decaying a user's lighting

use bevy::prelude::*;
use sf_core::{GameOver, Player};

pub fn lighting_decay(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(&mut Player, Entity)>,
) {
    let t = time.seconds_since_startup();

    for (mut player, entity) in query.iter_mut() {
        if player.lighting_strength == 0 || t < player.next_lighting_decay {
            continue;
        }

        player.next_lighting_decay = player.lighting_decay_rate + t;
        player.lighting_strength -= 1;

        if player.lighting_strength == 0 {
            println!("GAME OVER");
            commands.entity(entity).insert(GameOver);
        }
    }
}
