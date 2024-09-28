use super::components::Ball;
use super::NewBallOnPlatform;
use crate::lifes::LifeLost;
use crate::textures::HALF_TILE_SIZE;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

const MAX_SPEED: f32 = 500.0;
const MIN_SPEED: f32 = 300.0;
const MIN_Y_SPEED: f32 = 50.0;

pub fn confine_ball_speed(mut query: Query<(&mut Velocity, &Ball)>) {
    for (mut binding, ball) in query.iter_mut() {
        let velocity = binding.as_mut();
        let speed = velocity.linvel.length();

        //Confine max speed
        if speed > MAX_SPEED && !ball.lay_on_platform {
            velocity.linvel = velocity.linvel.normalize() * MAX_SPEED;
        }
        //confine min speed
        if speed < MIN_SPEED && !ball.lay_on_platform {
            velocity.linvel = velocity.linvel.normalize() * MIN_SPEED;
        }
        //prevent ball from getting stuck between walls
        if velocity.linvel.y.abs() < MIN_Y_SPEED && !ball.lay_on_platform {
            velocity.linvel.y = MIN_Y_SPEED * velocity.linvel.y.signum();
        }
    }
}

pub fn launch_ball(
    mut commands: Commands,
    mut ball_query: Query<(Entity, &mut Velocity, &mut Transform, &mut Ball)>,
    mut platform_query: Query<(Entity, &Transform, &Velocity), Without<Ball>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mouse_button: Res<ButtonInput<MouseButton>>,
) {
    for (ball_entity, mut velocity, mut ball_transform, mut ball) in ball_query.iter_mut() {
        if ball.lay_on_platform {
            let (platform_entity, platform_transform, platform_velocity) =
                platform_query.single_mut();
            if keyboard_input.just_pressed(KeyCode::ArrowUp)
                || keyboard_input.just_pressed(KeyCode::Space)
                || mouse_button.just_pressed(MouseButton::Left)
            {
                ball.lay_on_platform = false;
                velocity.linvel = Vec2::new(platform_velocity.linvel.x, 200.0);
                ball_transform.translation.y =
                    platform_transform.translation.y + ball.radius + HALF_TILE_SIZE;
                ball_transform.translation.x = platform_transform.translation.x;
                commands
                    .entity(platform_entity)
                    .remove_children(&[ball_entity]);
            }
        }
    }
}

pub fn despawn_balls(mut commands: Commands, ball_query: Query<Entity, With<Ball>>) {
    for entity in ball_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn check_balls_count(
    ball_query: Query<Entity, With<Ball>>,
    mut life_lost: EventWriter<LifeLost>,
    check_event: EventReader<NewBallOnPlatform>,
) {
    if ball_query.iter().count() == 0 && check_event.is_empty() {
        life_lost.send(LifeLost);
    }
}
