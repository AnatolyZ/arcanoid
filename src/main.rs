mod ball;
mod brick;
mod platform;
mod play_area;
mod textures;

use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

use ball::BallPlugin;
use brick::BricksPlugin;
use platform::PlatformPlugin;
use play_area::PlayAreaPlugin;
use textures::{TexturesPlugin, TILE_SIZE};

pub const SCREEN_HEIGHT_TILES: f32 = 36.0; // height of the game window in tiles
pub const SCREEN_WIDTH_TILES: f32 = 64.0; // width of the game window in tiles
pub const SCREEN_HEIGHT: f32 = SCREEN_HEIGHT_TILES * TILE_SIZE; // height of the game window in pixels
pub const SCREEN_WIDTH: f32 = SCREEN_WIDTH_TILES * TILE_SIZE; // width of the game window in pixels

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Arcanoid".into(),
                    resolution: (SCREEN_WIDTH, SCREEN_HEIGHT).into(),
                    ..default()
                }),
                ..default()
            }),
            LdtkPlugin,
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0),
            PlatformPlugin,
            BallPlugin,
            BricksPlugin,
            PlayAreaPlugin,
            TexturesPlugin,
            //RapierDebugRenderPlugin::default(),
        ))
        .run()
}
