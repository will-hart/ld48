use bevy::prelude::*;
use sf_core::TimedDespawn;

pub fn timed_despawner(
    mut commands: Commands,
    time: Res<Time>,
    mut items: Query<(&mut Timer, Entity), With<TimedDespawn>>,
) {
    for (mut timer, ent) in items.iter_mut() {
        timer.tick(time.delta());
        if timer.finished() {
            commands.entity(ent).despawn_recursive();
        }
    }
}
