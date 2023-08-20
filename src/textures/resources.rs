use bevy::prelude::*;

#[derive(Resource)]
pub struct Textures {
    pub ball: Handle<Image>,
    pub industrial: Handle<TextureAtlas>,
    pub sand: Handle<TextureAtlas>,
    pub marble: Handle<TextureAtlas>,
    pub rock: Handle<TextureAtlas>,
    pub stone: Handle<TextureAtlas>,
}
