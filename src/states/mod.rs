mod resources;
mod systems;

use bevy::prelude::*;

#[derive(Default, Copy, Clone, PartialEq, Eq, Hash, Debug, States)]
pub enum GameState {
    #[default]
    Menu,
    Setup,
    Game,
    Paused,
    Over,
}

pub struct StatesPlugin;

impl Plugin for StatesPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(resources::SetupTimer(Timer::from_seconds(
            1.0,
            TimerMode::Once,
        )))
        .add_systems(Update, systems::pause_game_control)
        .add_systems(OnEnter(GameState::Setup), systems::reset_setup_timer)
        .add_systems(
            Update,
            systems::tick_setup_timer.run_if(in_state(GameState::Setup)),
        )
        .add_systems(OnEnter(GameState::Paused), systems::on_enter_pause)
        .add_systems(OnEnter(GameState::Over), systems::on_enter_pause)
        .add_systems(OnExit(GameState::Paused), systems::on_exit_pause);
    }
}
