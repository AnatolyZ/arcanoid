use super::components::Animation;
use super::resources::AnimationTimer;
use crate::textures::{HALF_TILE_SIZE, TILE_SIZE};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::Rng;

pub fn switch_off_gravity(mut rapier_config: ResMut<RapierConfiguration>) {
    rapier_config.gravity = Vec2::ZERO;
}

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

pub fn spawn_borders(mut commands: Commands, windows: Query<&Window>) {
    let window = windows.single();
    let width = window.width();
    let height = window.height();
    let left = -width / 2.0 + TILE_SIZE / 2.0;
    let right = width / 2.0 - TILE_SIZE / 2.0;
    let top = height / 2.0 - TILE_SIZE / 2.0;
    let bottom = -height / 2.0 + TILE_SIZE / 2.0;

    commands.spawn((
        Collider::cuboid(width / 2.0, HALF_TILE_SIZE - 4.0),
        Transform::from_xyz(0.0, top, 0.0),
        GlobalTransform::default(),
        Friction {
            coefficient: 0.0,
            combine_rule: CoefficientCombineRule::Min,
        },
    ));
    commands.spawn((
        Collider::cuboid(width / 2.0, HALF_TILE_SIZE),
        Transform::from_xyz(0.0, bottom - TILE_SIZE, 0.0),
        GlobalTransform::default(),
        Friction {
            coefficient: 0.0,
            combine_rule: CoefficientCombineRule::Min,
        },
    ));
    commands.spawn((
        Collider::cuboid(HALF_TILE_SIZE - 4.0, height / 2.0),
        Transform::from_xyz(left, 0.0, 0.0),
        GlobalTransform::default(),
        Friction {
            coefficient: 0.0,
            combine_rule: CoefficientCombineRule::Min,
        },
    ));
    commands.spawn((
        Collider::cuboid(HALF_TILE_SIZE - 4.0, height / 2.0),
        Transform::from_xyz(right, 0.0, 0.0),
        GlobalTransform::default(),
        Friction {
            coefficient: 0.0,
            combine_rule: CoefficientCombineRule::Min,
        },
    ));
}

pub fn spawn_background(
    mut commands: Commands,
    assets_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    windows: Query<&Window>,
) {
    let window = windows.single();
    let width = window.width();
    let height = window.height();
    let left = -width / 2.0 + TILE_SIZE / 2.0;
    let right = width / 2.0 - TILE_SIZE / 2.0;
    let top = height / 2.0 - TILE_SIZE / 2.0;
    let bottom = -height / 2.0 + TILE_SIZE / 2.0;

    let texture_handle: Handle<Image> = assets_server.load("industrial_tiles.png");
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(TILE_SIZE, TILE_SIZE),
        16,
        7,
        None,
        None,
    );

    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands.spawn(SpriteSheetBundle {
        texture_atlas: texture_atlas_handle.clone(),
        sprite: TextureAtlasSprite::new(76),
        transform: Transform::from_xyz(left, top, 0.0),
        ..Default::default()
    });
    commands.spawn(SpriteSheetBundle {
        texture_atlas: texture_atlas_handle.clone(),
        sprite: TextureAtlasSprite::new(77),
        transform: Transform::from_xyz(right, top, 0.0),
        ..Default::default()
    });
    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle.clone(),
            sprite: TextureAtlasSprite::new(78),
            transform: Transform::from_xyz(left, bottom + TILE_SIZE, 1.0),
            ..Default::default()
        },
        Animation {
            phase: 0,
            sprites: vec![78, 79],
        },
    ));
    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle.clone(),
            sprite: TextureAtlasSprite::new(78),
            transform: Transform::from_xyz(right, bottom + TILE_SIZE, 1.0),
            ..Default::default()
        },
        Animation {
            phase: 0,
            sprites: vec![78, 79],
        },
    ));
    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle.clone(),
            sprite: TextureAtlasSprite::new(94),
            transform: Transform::from_xyz(left, bottom, 1.0),
            ..Default::default()
        },
        Animation {
            phase: 0,
            sprites: vec![94, 95],
        },
    ));
    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle.clone(),
            sprite: TextureAtlasSprite::new(94),
            transform: Transform::from_xyz(right, bottom, 1.0),
            ..Default::default()
        },
        Animation {
            phase: 0,
            sprites: vec![94, 95],
        },
    ));

    let mut rng = rand::thread_rng();

    // draw the top row
    for x in (left as i32 + TILE_SIZE as i32..right as i32).step_by(TILE_SIZE as usize) {
        let sprite = if rng.gen_range(0..15) == 0 { 110 } else { 109 };
        commands.spawn(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle.clone(),
            sprite: TextureAtlasSprite::new(sprite),
            transform: Transform::from_xyz(x as f32, top, 0.0),
            ..Default::default()
        });
    }
    // draw the bottom row
    for x in (left as i32 + TILE_SIZE as i32..right as i32).step_by(TILE_SIZE as usize) {
        commands.spawn((
            SpriteSheetBundle {
                texture_atlas: texture_atlas_handle.clone(),
                sprite: TextureAtlasSprite::new(13),
                transform: Transform::from_xyz(x as f32, bottom, 1.0),
                ..Default::default()
            },
            Animation {
                phase: 0,
                sprites: vec![13, 29],
            },
        ));
    }
    // draw sides
    for y in (bottom as i32 + (TILE_SIZE * 2.0) as i32..top as i32).step_by(TILE_SIZE as usize) {
        let sprite = if rng.gen_range(0..15) == 0 { 91 } else { 75 };
        commands.spawn(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle.clone(),
            sprite: TextureAtlasSprite::new(sprite),
            transform: Transform::from_xyz(left, y as f32, 0.0),
            ..Default::default()
        });
        let sprite = if rng.gen_range(0..15) == 0 { 91 } else { 75 };
        commands.spawn(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle.clone(),
            sprite: TextureAtlasSprite::new(sprite),
            transform: Transform::from_xyz(right, y as f32, 0.0),
            ..Default::default()
        });
    }
}

pub fn tick_animation(
    time: Res<Time>,
    mut timer: ResMut<AnimationTimer>,
    mut query: Query<(&mut Animation, &mut TextureAtlasSprite)>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        for (mut animation, mut sprite) in query.iter_mut() {
            animation.phase += 1;
            animation.phase = animation.phase % animation.sprites.len();
            sprite.index = animation.sprites[animation.phase];
        }
    }
}
