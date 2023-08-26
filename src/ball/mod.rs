mod components;
mod systems;

use bevy::prelude::*;
pub use components::Ball;

const MAX_X_SPEED: f32 = 300.0;
const MAX_Y_SPEED: f32 = 300.0;
const MIN_X_SPEED: f32 = 100.0;
const MIM_Y_SPEED: f32 = 100.0;

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, systems::spawn_ball)
            .add_systems(Update, (systems::confine_ball_speed, systems::launch_ball));
    }
}
