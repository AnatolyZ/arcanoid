use super::{resources::Textures, BRICK_TILE_SIZE, TILE_SIZE};
use bevy::prelude::*;

pub fn load_textures(
    mut commands: Commands,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    assets_server: Res<AssetServer>,
) {
    let ball: Handle<Image> = assets_server.load("ball_red_small.png");

    let texture_handle: Handle<Image> = assets_server.load("industrial_tiles.png");
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(TILE_SIZE, TILE_SIZE),
        16,
        7,
        None,
        None,
    );
    let industrial = texture_atlases.add(texture_atlas);

    let texture_handle: Handle<Image> = assets_server.load("marble_tiles.png");
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(BRICK_TILE_SIZE, BRICK_TILE_SIZE),
        9,
        9,
        None,
        None,
    );
    let marble = texture_atlases.add(texture_atlas);

    let texture_handle: Handle<Image> = assets_server.load("rock_tiles.png");
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(BRICK_TILE_SIZE, BRICK_TILE_SIZE),
        9,
        9,
        None,
        None,
    );
    let rock = texture_atlases.add(texture_atlas);

    let texture_handle: Handle<Image> = assets_server.load("sand_tiles.png");
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(BRICK_TILE_SIZE, BRICK_TILE_SIZE),
        3,
        3,
        None,
        None,
    );
    let sand = texture_atlases.add(texture_atlas);

    let texture_handle: Handle<Image> = assets_server.load("stone_tiles.png");
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(BRICK_TILE_SIZE, BRICK_TILE_SIZE),
        9,
        9,
        None,
        None,
    );
    let stone = texture_atlases.add(texture_atlas);

    commands.insert_resource(Textures {
        ball,
        industrial,
        marble,
        rock,
        sand,
        stone,
    });
}
