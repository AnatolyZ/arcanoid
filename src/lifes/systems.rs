use crate::ball::NewBallOnPlatform;
use crate::states::GameState;
use bevy::log;
use bevy::prelude::*;
use bevy_ecs_ldtk::assets::LevelIndices;
use bevy_ecs_ldtk::LevelSelection;

use super::components::Lifes;
use super::resources::LifesSettings;
use super::LifeLost;

pub fn spawn_lifes(mut commands: Commands, lifes_settings: Res<LifesSettings>) {
    commands.spawn(Lifes::new(lifes_settings.initial_lifes, 10));
    log::info!("Lifes spawned with {} lifes", lifes_settings.initial_lifes);
}

pub fn despawn_lifes(mut commands: Commands, query: Query<Entity, With<Lifes>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn update_lifes_count(
    mut lifes_query: Query<&mut Lifes>,
    mut life_lost_events: EventReader<LifeLost>,
    mut new_ball_event: EventWriter<NewBallOnPlatform>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let mut lifes = lifes_query.single_mut();
    for _ in life_lost_events.read() {
        lifes.decrease(1);
        new_ball_event.send(NewBallOnPlatform);
        if lifes.is_empty() {
            next_state.set(GameState::OverOver);
        }
    }
}

pub fn reset_lifes(
    mut lifes_query: Query<&mut Lifes>,
    mut level_selection: ResMut<LevelSelection>,
) {
    let mut lifes = lifes_query.single_mut();
    lifes.reset();
    *level_selection = LevelSelection::Indices(LevelIndices::in_root(0));
}
