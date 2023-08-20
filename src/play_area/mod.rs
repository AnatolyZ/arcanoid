mod components;
mod resources;
mod systems;

use bevy::prelude::*;

pub struct PlayAreaPlugin;

impl Plugin for PlayAreaPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(resources::AnimationTimer(Timer::from_seconds(
            1.0,
            TimerMode::Repeating,
        )))
        .add_systems(
            Startup,
            (
                systems::spawn_camera,
                systems::switch_off_gravity,
                systems::spawn_borders,
                systems::spawn_background,
            ),
        )
        .add_systems(Update, systems::tick_animation);
    }
}
