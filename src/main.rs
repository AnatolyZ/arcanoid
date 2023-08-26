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

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Arcanoid".into(),
                    resolution: (TILE_SIZE * 64.0, TILE_SIZE * 36.0).into(),
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
