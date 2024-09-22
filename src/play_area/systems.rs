use super::components::Animation;
use super::components::Background;
use super::components::Border;
use super::components::LdtkWorld;
use super::components::MainCamera;
use super::components::OutSensor;
use super::resources::AnimationTimer;
use crate::states::GameState;
use crate::{
    textures::{self, HALF_TILE_SIZE, TILE_SIZE},
    SCREEN_HEIGHT, SCREEN_WIDTH,
};
use bevy::math::bool;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;
use textures::resources::Textures;

pub fn switch_off_gravity(mut rapier_config: ResMut<RapierConfiguration>) {
    rapier_config.gravity = Vec2::ZERO;
}

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), MainCamera));
}

pub fn spawn_borders(mut commands: Commands, windows: Query<&Window>) {
    let window = windows.single();
    let width = window.width();
    let height = window.height();
    let left = -width / 2.0 + TILE_SIZE / 2.0 + TILE_SIZE * 6.0;
    let right = width / 2.0 - TILE_SIZE / 2.0 - TILE_SIZE * 6.0;
    let top = height / 2.0 - TILE_SIZE / 2.0;
    let bottom = -height / 2.0 + TILE_SIZE / 2.0;

    commands.spawn((
        Border {},
        Collider::cuboid(width / 2.0, HALF_TILE_SIZE),
        Transform::from_xyz(0.0, top, 2.0),
        GlobalTransform::default(),
        Friction {
            coefficient: 0.1,
            combine_rule: CoefficientCombineRule::Min,
        },
    ));
    commands.spawn((
        Border {},
        Collider::cuboid(HALF_TILE_SIZE, height / 2.0),
        Transform::from_xyz(right, 0.0, 2.0),
        GlobalTransform::default(),
        Friction {
            coefficient: 0.0,
            combine_rule: CoefficientCombineRule::Min,
        },
    ));
    commands.spawn((
        Border {},
        Collider::cuboid(HALF_TILE_SIZE, height / 2.0),
        Transform::from_xyz(left, 0.0, 2.0),
        GlobalTransform::default(),
        Friction {
            coefficient: 0.1,
            combine_rule: CoefficientCombineRule::Min,
        },
    ));
    commands.spawn((
        Border {},
        Collider::cuboid(width / 2.0, HALF_TILE_SIZE),
        Transform::from_xyz(0.0, bottom - TILE_SIZE * 2.0, 2.0),
        GlobalTransform::default(),
        ActiveEvents::COLLISION_EVENTS,
        Friction {
            coefficient: 0.0,
            combine_rule: CoefficientCombineRule::Min,
        },
        Sensor,
        OutSensor {},
    ));
}

pub fn spawn_background(mut commands: Commands, textures: Res<Textures>) {
    let left = -SCREEN_WIDTH / 2.0 + TILE_SIZE / 2.0;
    let right = SCREEN_WIDTH / 2.0 - TILE_SIZE / 2.0;
    let bottom = -SCREEN_HEIGHT / 2.0 + TILE_SIZE / 2.0;
    let top = SCREEN_HEIGHT / 2.0 - TILE_SIZE / 2.0;

    fn draw_single_sprite(
        commands: &mut Commands,
        textures: &Res<Textures>,
        sprite_index: usize,
        x: f32,
        y: f32,
        flip_x: bool,
        flip_y: bool,
    ) {
        commands.spawn((
            Background {},
            SpriteBundle {
                texture: textures.industrial.texture.clone(),
                transform: Transform::from_xyz(x, y, 2.0),
                sprite: Sprite {
                    flip_x,
                    flip_y,
                    ..Default::default()
                },
                ..Default::default()
            },
            TextureAtlas {
                layout: textures.industrial.layout.clone(),
                index: sprite_index,
            },
        ));
    }

    fn draw_vertical_line(
        commands: &mut Commands,
        textures: &Res<Textures>,
        sprite_index: usize,
        x: f32,
        bottom: f32,
        top: f32,
    ) {
        for y in (bottom as i32..top as i32 + TILE_SIZE as i32).step_by(TILE_SIZE as usize) {
            commands.spawn((
                Background {},
                SpriteBundle {
                    texture: textures.industrial.texture.clone(),
                    transform: Transform::from_xyz(x, y as f32, 2.0),
                    ..Default::default()
                },
                TextureAtlas {
                    layout: textures.industrial.layout.clone(),
                    index: sprite_index,
                },
            ));
        }
    }

    fn draw_horizontal_line(
        commands: &mut Commands,
        textures: &Res<Textures>,
        sprite_index: usize,
        y: f32,
        left: f32,
        right: f32,
    ) {
        for x in (left as i32..right as i32 + TILE_SIZE as i32).step_by(TILE_SIZE as usize) {
            commands.spawn((
                Background {},
                SpriteBundle {
                    texture: textures.industrial.texture.clone(),
                    transform: Transform::from_xyz(x as f32, y, 2.0),
                    ..Default::default()
                },
                TextureAtlas {
                    layout: textures.industrial.layout.clone(),
                    index: sprite_index,
                },
            ));
        }
    }

    // sedes of the screen panels
    draw_vertical_line(
        &mut commands,
        &textures,
        52,
        right,
        bottom + TILE_SIZE * 8.0,
        top - TILE_SIZE,
    );
    draw_vertical_line(
        &mut commands,
        &textures,
        52,
        left,
        bottom + TILE_SIZE * 8.0,
        top - TILE_SIZE,
    );

    // verticel pipes
    draw_vertical_line(
        &mut commands,
        &textures,
        75,
        left,
        bottom + TILE_SIZE * 2.0,
        bottom + TILE_SIZE * 6.0,
    );
    draw_vertical_line(
        &mut commands,
        &textures,
        75,
        right,
        bottom + TILE_SIZE * 2.0,
        bottom + TILE_SIZE * 6.0,
    );

    // vertical panels for ball area
    draw_vertical_line(
        &mut commands,
        &textures,
        52,
        right - TILE_SIZE * 6.0,
        bottom + TILE_SIZE,
        top - TILE_SIZE,
    );
    draw_vertical_line(
        &mut commands,
        &textures,
        52,
        left + TILE_SIZE * 6.0,
        bottom + TILE_SIZE,
        top - TILE_SIZE,
    );

    // top joints
    draw_single_sprite(
        &mut commands,
        &textures,
        38,
        right - TILE_SIZE * 6.0,
        top,
        false,
        false,
    );
    draw_single_sprite(
        &mut commands,
        &textures,
        38,
        left + TILE_SIZE * 6.0,
        top,
        false,
        false,
    );
    draw_single_sprite(&mut commands, &textures, 38, right, top, false, false);
    draw_single_sprite(&mut commands, &textures, 38, left, top, false, false);

    // top panels
    draw_horizontal_line(
        &mut commands,
        &textures,
        5,
        top,
        left + TILE_SIZE,
        left + TILE_SIZE * 5.0,
    );
    draw_horizontal_line(
        &mut commands,
        &textures,
        5,
        top,
        left + TILE_SIZE * 7.0,
        right - TILE_SIZE * 7.0,
    );
    draw_horizontal_line(
        &mut commands,
        &textures,
        5,
        top,
        right - TILE_SIZE * 5.0,
        right - TILE_SIZE,
    );

    // bottom panels
    draw_horizontal_line(
        &mut commands,
        &textures,
        5,
        bottom + TILE_SIZE * 7.0,
        left + TILE_SIZE,
        left + TILE_SIZE * 5.0,
    );
    draw_horizontal_line(
        &mut commands,
        &textures,
        5,
        bottom + TILE_SIZE * 7.0,
        right - TILE_SIZE * 5.0,
        right - TILE_SIZE,
    );

    // bottom joints
    draw_single_sprite(
        &mut commands,
        &textures,
        38,
        left,
        bottom + TILE_SIZE * 7.0,
        false,
        true,
    );
    draw_single_sprite(
        &mut commands,
        &textures,
        38,
        right,
        bottom + TILE_SIZE * 7.0,
        false,
        true,
    );

    // bottom details
    draw_single_sprite(
        &mut commands,
        &textures,
        80,
        right - TILE_SIZE * 5.0,
        bottom + TILE_SIZE * 6.0,
        false,
        false,
    );
    draw_single_sprite(
        &mut commands,
        &textures,
        96,
        left + TILE_SIZE * 5.0,
        bottom + TILE_SIZE * 6.0,
        false,
        false,
    );

    // draw animated drainings
    commands.spawn((
        Background {},
        SpriteBundle {
            texture: textures.industrial.texture.clone(),
            transform: Transform::from_xyz(left, bottom + TILE_SIZE, 2.0),
            ..Default::default()
        },
        TextureAtlas {
            layout: textures.industrial.layout.clone(),
            index: 78,
        },
        Animation {
            phase: 0,
            sprites: vec![78, 79],
        },
    ));
    commands.spawn((
        Background {},
        SpriteBundle {
            texture: textures.industrial.texture.clone(),
            transform: Transform::from_xyz(right, bottom + TILE_SIZE, 2.0),
            ..Default::default()
        },
        TextureAtlas {
            layout: textures.industrial.layout.clone(),
            index: 78,
        },
        Animation {
            phase: 0,
            sprites: vec![78, 79],
        },
    ));
    commands.spawn((
        Background {},
        SpriteBundle {
            texture: textures.industrial.texture.clone(),
            transform: Transform::from_xyz(left, bottom, 2.0),
            ..Default::default()
        },
        TextureAtlas {
            layout: textures.industrial.layout.clone(),
            index: 94,
        },
        Animation {
            phase: 0,
            sprites: vec![94, 95],
        },
    ));
    commands.spawn((
        Background {},
        SpriteBundle {
            texture: textures.industrial.texture.clone(),
            transform: Transform::from_xyz(right, bottom, 2.0),
            ..Default::default()
        },
        TextureAtlas {
            layout: textures.industrial.layout.clone(),
            index: 94,
        },
        Animation {
            phase: 0,
            sprites: vec![94, 95],
        },
    ));

    for x in (left as i32 + TILE_SIZE as i32..right as i32).step_by(TILE_SIZE as usize) {
        commands.spawn((
            Background {},
            SpriteBundle {
                texture: textures.industrial.texture.clone(),
                transform: Transform::from_xyz(x as f32, bottom, 4.0),
                ..Default::default()
            },
            TextureAtlas {
                layout: textures.industrial.layout.clone(),
                index: 13,
            },
            Animation {
                phase: 0,
                sprites: vec![13, 29],
            },
        ));
    }
}

pub fn tick_animation(
    time: Res<Time>,
    mut timer: ResMut<AnimationTimer>,
    mut query: Query<(&mut Animation, &mut TextureAtlas)>,
) {
    if timer.tick(time.delta()).just_finished() {
        for (mut animation, mut sprite) in query.iter_mut() {
            animation.phase += 1;
            animation.phase %= animation.sprites.len();
            sprite.index = animation.sprites[animation.phase];
        }
    }
}

pub fn load_ldtk_world(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        LdtkWorld {},
        LdtkWorldBundle {
            // TODO: Load only once
            ldtk_handle: asset_server.load("levels.ldtk"),
            transform: Transform::from_xyz(-SCREEN_WIDTH / 2.0, -SCREEN_HEIGHT / 2.0, -10.0),
            ..Default::default()
        },
    ));
}

pub fn collision_handler(
    mut commands: Commands,
    mut collisions: EventReader<CollisionEvent>,
    mut out_sensor_query: Query<Entity, With<OutSensor>>,
) {
    for ev in collisions.read() {
        if let CollisionEvent::Started(entity1, entity2, _) = ev {
            if out_sensor_query.get_mut(*entity1).is_ok() {
                // entity1 is the out sensor, so entity2 is the ball
                commands.entity(*entity2).despawn_recursive();
            }
            if out_sensor_query.get_mut(*entity2).is_ok() {
                // entity2 is the out sensor, so entity1 is the ball
                commands.entity(*entity1).despawn_recursive();
            }
        }
    }
}

pub fn despawn_borders(mut commands: Commands, border_query: Query<Entity, With<Border>>) {
    for entity in border_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn despawn_background(
    mut commands: Commands,
    background_query: Query<Entity, With<Background>>,
) {
    for entity in background_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn despawn_ldtk_world(
    mut commands: Commands,
    ldtk_world_query: Query<Entity, With<LdtkWorld>>,
) {
    for entity in ldtk_world_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn transition_to_menu(mut app_state: ResMut<NextState<GameState>>) {
    app_state.set(GameState::Menu);
}
