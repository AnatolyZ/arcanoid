use super::GameState;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub fn pause_game_control(
    app_state: ResMut<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        match app_state.get() {
            GameState::Game => next_state.set(GameState::Paused),
            GameState::Paused => next_state.set(GameState::Game),
            GameState::Menu => todo!(),
            GameState::Over => todo!(),
        }
    }
}

pub fn on_enter_pause(mut rapier_config: ResMut<RapierConfiguration>) {
    rapier_config.physics_pipeline_active = false;
}

pub fn on_exit_pause(mut rapier_config: ResMut<RapierConfiguration>) {
    rapier_config.physics_pipeline_active = true;
}
