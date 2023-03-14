use crate::config::{CELL_HEIGHT, CELL_WIDTH, HALF_HEIGHT, HALF_WIDTH};
use bevy::prelude::*;

/// 'z' is a layer
#[derive(Component, Copy, Clone)]
pub struct Position {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Position {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }
}

pub struct GroundPlugin;

impl Plugin for GroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(CoordSystem::translate);
    }
}

pub struct CoordSystem;

impl CoordSystem {
    /// Translates the coordinate system
    pub fn translate(mut entities: Query<(&mut Transform, &Sprite, &Position), With<Position>>) {
        entities.for_each_mut(|(mut transform, sprite, position)| {
            if let Some(size) = sprite.custom_size {
                let (x, y): (f32, f32) = Self::get_translate(&position, &size);

                transform.translation.x = x;
                transform.translation.y = y;
            }
        });
    }

    /// Returns the translation of the coordinate system
    pub fn get_translate(position: &Position, size: &Vec2) -> (f32, f32) {
        let half_scale_x: f32 = size.x / 2.;
        let half_scale_y: f32 = size.y / 2.;

        let x: f32 = (position.x as f32 * CELL_WIDTH) - HALF_WIDTH + half_scale_x;
        let y: f32 = (position.y as f32 * CELL_HEIGHT) - HALF_HEIGHT + half_scale_y;

        (x, y)
    }

    pub fn in_one_position(first: &Position, other: &Position) -> bool {
        first.x == other.x && first.y == other.y
    }
}
