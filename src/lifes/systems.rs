use bevy::prelude::*;
use super::components::Lifes;
use super::resources::LifesSettings;

pub fn spawn_lifes(mut commands: Commands, lifes_settings: Res<LifesSettings>) {
    commands.spawn(Lifes(lifes_settings.initial_lifes));
}

pub fn despawn_lifes(mut commands: Commands, query: Query<Entity, With<Lifes>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}