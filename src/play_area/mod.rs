mod components;
mod resources;
mod systems;

use crate::states::GameState;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

pub struct PlayAreaPlugin;

impl Plugin for PlayAreaPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(resources::AnimationTimer(Timer::from_seconds(
            1.0,
            TimerMode::Repeating,
        )))
        .insert_resource(LevelSelection::Index(0))
        .add_systems(
            Startup,
            (
                systems::spawn_camera,
                systems::switch_off_gravity,
                systems::spawn_borders,
                systems::spawn_background,
                systems::load_ldtk_level,
            ),
        )
        .add_systems(
            Update,
            systems::tick_animation.run_if(in_state(GameState::Game)),
        );
    }
}
