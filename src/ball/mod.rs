pub(crate) mod components;
mod events;
mod systems;

use crate::states::GameState;
use bevy::prelude::*;
pub use components::Ball;
pub use events::NewBallOnPlatform;

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<NewBallOnPlatform>()
            .add_systems(
                Update,
                (
                    systems::confine_ball_speed,
                    systems::launch_ball,
                    systems::check_balls_count,
                )
                    .run_if(in_state(GameState::Game)),
            )
            .add_systems(OnExit(GameState::OverOver), (systems::despawn_balls,))
            .add_systems(OnExit(GameState::LevelComplete), (systems::despawn_balls,))
            .add_systems(OnEnter(GameState::Menu), (systems::despawn_balls,));
    }
}
