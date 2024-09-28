mod components;
mod resources;
mod systems;

use crate::states::GameState;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

pub use components::MainCamera;
pub use components::Border;

pub struct PlayAreaPlugin;

impl Plugin for PlayAreaPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(resources::AnimationTimer(Timer::from_seconds(
            0.25,
            TimerMode::Repeating,
        )))
        .insert_resource(LevelSelection::Indices(LevelIndices::in_root(0)))
        .add_systems(OnEnter(GameState::SpawnCamera), systems::spawn_camera)
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
                systems::transition_to_menu.run_if(in_state(GameState::SpawnCamera)),
                systems::tick_animation.run_if(in_state(GameState::Game)),
                systems::collision_handler.run_if(in_state(GameState::Game)),
            ),
        )
        .add_systems(
            OnExit(GameState::OverOver),
            (
                systems::despawn_background,
                systems::despawn_borders,
                systems::despawn_ldtk_world,
            ),
        )
        .add_systems(
            OnEnter(GameState::Menu),
            (
                systems::despawn_background,
                systems::despawn_borders,
                systems::despawn_ldtk_world,
            ),
        )
        .add_systems(
            OnExit(GameState::LevelComplete),
            (
                systems::despawn_background,
                systems::despawn_borders,
                systems::despawn_ldtk_world,
            ),
        );
    }
}
