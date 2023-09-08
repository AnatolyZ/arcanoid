mod components;
mod systems;

use bevy::prelude::*;
pub use components::Ball;

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, systems::spawn_ball)
            .add_systems(Update, (systems::confine_ball_speed, systems::launch_ball));
    }
}
