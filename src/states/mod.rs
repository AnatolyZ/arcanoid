mod resources;
mod systems;

use bevy::prelude::*;

#[derive(Default, Copy, Clone, PartialEq, Eq, Hash, Debug, States)]
pub enum GameState {
    #[default]
    LoadTextures,
    SpawnCamera,
    Menu,
    Setup,
    Game,
    Paused,
    OverOver,
    LevelComplete,
}

pub struct StatesPlugin;

impl Plugin for StatesPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(resources::SetupTimer(Timer::from_seconds(
            0.1,
            TimerMode::Once,
        )))
        .add_systems(
            Update,
            systems::pause_game_control.run_if(|current_state: Res<State<GameState>>| {
                *current_state == GameState::Game || *current_state == GameState::Paused
            }),
        )
        .add_systems(OnEnter(GameState::Setup), systems::reset_setup_timer)
        .add_systems(
            Update,
            systems::tick_setup_timer.run_if(in_state(GameState::Setup)),
        )
        .add_systems(OnEnter(GameState::Game), systems::on_enter_game)
        .add_systems(OnExit(GameState::Game), systems::on_exit_game);
    }
}
