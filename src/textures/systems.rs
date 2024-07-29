use crate::states::GameState;

use super::{
    resources::{SpriteSheet, Textures},
    BRICK_TILE_SIZE, TILE_SIZE,
};
use bevy::prelude::*;

pub fn load_textures(
    mut commands: Commands,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    assets_server: Res<AssetServer>,
) {
    let ball: Handle<Image> = assets_server.load("ball_red_small.png");
    let bevy_logo: Handle<Image> = assets_server.load("bevy_logo.png");
    let rapier_logo: Handle<Image> = assets_server.load("rapier_logo.png");

    let background: Handle<Image> = assets_server.load("background.png");

    let industrial_texture_handle: Handle<Image> = assets_server.load("industrial_tiles.png");
    let texture_atlas = TextureAtlasLayout::from_grid(
        UVec2::new(TILE_SIZE as u32, TILE_SIZE as u32),
        16,
        7,
        None,
        None,
    );
    let industrial_layout = texture_atlases.add(texture_atlas);

    let marble_texture_handle: Handle<Image> = assets_server.load("marble_tiles.png");
    let texture_atlas = TextureAtlasLayout::from_grid(
        UVec2::new(BRICK_TILE_SIZE as u32, BRICK_TILE_SIZE as u32),
        3,
        3,
        None,
        None,
    );
    let marble_layout = texture_atlases.add(texture_atlas);

    let rock_texture_handle: Handle<Image> = assets_server.load("rock_tiles.png");
    let texture_atlas = TextureAtlasLayout::from_grid(
        UVec2::new(BRICK_TILE_SIZE as u32, BRICK_TILE_SIZE as u32),
        3,
        3,
        None,
        None,
    );
    let rock_layout = texture_atlases.add(texture_atlas);

    let sand_texture_handle: Handle<Image> = assets_server.load("sand_tiles.png");
    let texture_atlas = TextureAtlasLayout::from_grid(
        UVec2::new(BRICK_TILE_SIZE as u32, BRICK_TILE_SIZE as u32),
        3,
        3,
        None,
        None,
    );
    let sand_layout = texture_atlases.add(texture_atlas);

    let stone_texture_handle: Handle<Image> = assets_server.load("stone_tiles.png");
    let texture_atlas = TextureAtlasLayout::from_grid(
        UVec2::new(BRICK_TILE_SIZE as u32, BRICK_TILE_SIZE as u32),
        3,
        3,
        None,
        None,
    );
    let stone_layout = texture_atlases.add(texture_atlas);

    let font: Handle<Font> = assets_server.load("Broshk.otf");

    commands.insert_resource(Textures {
        ball,
        background,
        industrial: SpriteSheet {
            layout: industrial_layout,
            texture: industrial_texture_handle,
        },
        marble: SpriteSheet {
            layout: marble_layout,
            texture: marble_texture_handle,
        },
        rock: SpriteSheet {
            layout: rock_layout,
            texture: rock_texture_handle,
        },
        sand: SpriteSheet {
            layout: sand_layout,
            texture: sand_texture_handle,
        },
        stone: SpriteSheet {
            layout: stone_layout,
            texture: stone_texture_handle,
        },
        font,
        bevy_logo,
        rapier_logo,
    });
}

pub fn transition_to_spawn_camera(mut app_state: ResMut<NextState<GameState>>) {
    app_state.set(GameState::SpawnCamera);
}
