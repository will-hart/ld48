// handles periodically decaying a user's lighting

use bevy::prelude::*;
use sf_core::{LightingTarget, Player};

pub fn lighting_decay(time: Res<Time>, mut query: Query<&mut LightingTarget, With<Player>>) {
    let t = time.seconds_since_startup();

    for mut light in query.iter_mut() {
        if light.lighting_strength == 0 || t < light.next_lighting_decay {
            continue;
        }

        light.next_lighting_decay = light.lighting_decay_rate + t;
        light.lighting_strength -= 1;
    }
}
