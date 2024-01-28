mod components;
mod resources;
mod systems;

pub use components::LdtkWorld;

use crate::states::GameState;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

pub struct PlayAreaPlugin;

impl Plugin for PlayAreaPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(resources::AnimationTimer(Timer::from_seconds(
            0.25,
            TimerMode::Repeating,
        )))
        .insert_resource(LevelSelection::Index(0))
        .add_systems(Startup, systems::spawn_camera)
        .add_systems(
            OnEnter(GameState::Setup),
            (
                systems::switch_off_gravity,
                systems::spawn_borders,
                systems::spawn_background,
                systems::load_ldtk_world,
            ),
        )
        .add_systems(
            Update,
            (
                systems::tick_animation.run_if(in_state(GameState::Game)),
                systems::collision_handler.run_if(in_state(GameState::Game)),
            ),
        )
        .add_systems(
            OnExit(GameState::Over),
            (
                systems::despawn_background,
                systems::despawn_borders,
                systems::despawn_ldtk_world,
            ),
        );
    }
}
