use bevy::prelude::*;

#[derive(Resource, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);
