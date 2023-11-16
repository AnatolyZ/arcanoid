use bevy::prelude::*;

#[derive(Resource)]
pub struct Textures {
    pub ball: Handle<Image>,
    pub background: Handle<Image>,
    pub industrial: Handle<TextureAtlas>,
    pub sand: Handle<TextureAtlas>,
    pub marble: Handle<TextureAtlas>,
    pub rock: Handle<TextureAtlas>,
    pub stone: Handle<TextureAtlas>,
    pub font: Handle<Font>,
    pub bevy_logo: Handle<Image>,
    pub rapier_logo: Handle<Image>,
}
