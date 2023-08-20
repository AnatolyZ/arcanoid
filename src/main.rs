mod ball;
mod platform;
mod play_area;
mod textures;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use ball::BallPlugin;
use platform::PlatformPlugin;
use play_area::PlayAreaPlugin;
use textures::{TexturesPlugin, TILE_SIZE};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Arcanoid".into(),
                    resolution: (TILE_SIZE * 40.0, TILE_SIZE * 30.0).into(),
                    ..default()
                }),
                ..default()
            }),
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0),
            PlatformPlugin,
            BallPlugin,
            PlayAreaPlugin,
            TexturesPlugin,
            //RapierDebugRenderPlugin::default(),
        ))
        .run()
}
