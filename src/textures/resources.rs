use bevy::prelude::*;

#[derive(Clone)]
pub struct SpriteSheet {
    pub layout: Handle<TextureAtlasLayout>,
    pub texture: Handle<Image>,
}

#[derive(Resource)]
pub struct Textures {
    pub ball: Handle<Image>,
    pub background: Handle<Image>,
    pub industrial: SpriteSheet,
    pub sand: SpriteSheet,
    pub marble: SpriteSheet,
    pub rock: SpriteSheet,
    pub stone: SpriteSheet,
    pub font: Handle<Font>,
    pub bevy_logo: Handle<Image>,
    pub rapier_logo: Handle<Image>,
}
