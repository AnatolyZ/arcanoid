use super::GameState;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub fn pause_game_control(
    app_state: ResMut<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        if *app_state.get() == GameState::Game {
            next_state.set(GameState::Paused);
        } else {
            next_state.set(GameState::Game);
        }
    }
}

pub fn on_enter_game(mut rapier_config: ResMut<RapierConfiguration>) {
    rapier_config.physics_pipeline_active = true;
}

pub fn on_exit_game(mut rapier_config: ResMut<RapierConfiguration>) {
    rapier_config.physics_pipeline_active = false;
}
