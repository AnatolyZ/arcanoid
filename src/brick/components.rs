use crate::textures::HALF_BRICK_TILE_SIZE;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Default)]
pub enum BrickType {
    #[default]
    Sand,
    Stone,
    Rock,
    Marble,
}

#[derive(Default, Component, LdtkIntCell)]
pub struct Brick {
    pub resistance: u8,
    pub inhibition_rate: f32,
    pub brick_type: BrickType,
    pub health: Health,
    pub collision_sound: Handle<AudioSource>,
    pub destruction_sound: Handle<AudioSource>,
}

#[derive(Default, Component)]
pub struct Health(pub i32); // Health of the brick in percentage

#[derive(Bundle, LdtkIntCell)]
pub struct BrickBundle {
    #[from_int_grid_cell]
    pub brick: Brick,
    #[from_int_grid_cell]
    pub collider_bundle: ColliderBundle,
}

#[derive(Bundle, Default, LdtkIntCell)]
pub struct ColliderBundle {
    pub collider: Collider,
    pub rigid_body: RigidBody,
    pub active_events: ActiveEvents,
}

impl From<IntGridCell> for ColliderBundle {
    fn from(_cell: IntGridCell) -> Self {
        Self {
            collider: Collider::cuboid(HALF_BRICK_TILE_SIZE, HALF_BRICK_TILE_SIZE),
            rigid_body: RigidBody::Fixed,
            active_events: ActiveEvents::COLLISION_EVENTS,
        }
    }
}

impl From<IntGridCell> for Brick {
    fn from(int_grid_cell: IntGridCell) -> Brick {
        match int_grid_cell.value {
            1 => Brick {
                resistance: 3,
                inhibition_rate: 1.,
                brick_type: BrickType::Sand,
                health: Health(100),
                ..Default::default()
            },
            2 => Brick {
                resistance: 8,
                inhibition_rate: 1.,
                brick_type: BrickType::Stone,
                health: Health(100),
                ..Default::default()
            },
            3 => Brick {
                resistance: 15,
                inhibition_rate: 1.,
                brick_type: BrickType::Rock,
                health: Health(100),
                ..Default::default()
            },
            4 => Brick {
                resistance: 25,
                inhibition_rate: 1.,
                brick_type: BrickType::Marble,
                health: Health(100),
                ..Default::default()
            },
            _ => Brick::default(),
        }
    }
}
