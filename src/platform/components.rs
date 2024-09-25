use bevy::prelude::*;

#[derive(Component)]
pub struct Platform {
    pub collision_sound: Handle<AudioSource>,
}
