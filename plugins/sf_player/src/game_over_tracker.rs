use bevy::prelude::*;
use sf_core::{GameOver, GameState, LightingTarget, Player, Position};

pub fn game_over_tracker(
    mut commands: Commands,
    mut state: ResMut<State<GameState>>,
    players: Query<(&Position, &LightingTarget, &Player)>,
) {
    let (pos, light, player) = players.single().expect("Should have a player");

    // check if the light has expired
    if light.lighting_strength == 0 {
        println!("DEFEAT");
        commands.spawn().insert(GameOver);
        state.set(GameState::GameOver).unwrap();
    }

    // check if the player has hit the ground
    if pos.1 <= 0 {
        if player.slime_target == 0 {
            println!("VICTORY");
            state.set(GameState::Loading).unwrap();
        } else {
            println!("DEFEAT");
            commands.spawn().insert(GameOver);
            state.set(GameState::GameOver).unwrap();
        }
    }
}
