use bevy::prelude::*;
use sf_core::{dims::Dims, Position};

pub fn sync_transform_to_pos(
    dims: Res<Dims>,
    mut player_query: Query<(&mut Position, &Transform)>,
) {
    for (mut pos, tx) in player_query.iter_mut() {
        let (x, y) = dims.world_to_grid(tx.translation.truncate());
        pos.0 = x;
        pos.1 = y;
    }
}
