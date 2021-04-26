use bevy::prelude::*;
use sf_core::{GameOver, GameState, levels::NextLevel, Player, Position};

pub fn game_over_tracker(
    mut commands: Commands,
    mut state: ResMut<State<GameState>>,
    players: Query<(&Position, &Player)>,
) {
    let (pos, player) = players.single().expect("Should have a player");
    if pos.1 != 0 {
        return;
    }

    if player.slime_target == 0 {
        println!("VICTORY");
        commands.spawn().insert(NextLevel(2));
        state.set(GameState::Loading).unwrap();
    } else {
        println!("DEFEAT");
        commands.spawn().insert(GameOver);
        state.set(GameState::GameOver).unwrap();
    }
}
