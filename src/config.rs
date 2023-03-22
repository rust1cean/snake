use bevy::prelude::Color;
use std::ops::Range;

// Window
pub(crate) const TITLE: &str = "Snake";
pub(crate) const WIDTH: f32 = 300.;
pub(crate) const HEIGHT: f32 = WIDTH;
pub(crate) const HALF_WIDTH: f32 = WIDTH / 2.;
pub(crate) const HALF_HEIGHT: f32 = HEIGHT / 2.;

// General
pub(crate) const FRAMERATE: f32 = 1. / 4.;

// Grid
pub(crate) const GRID_WIDTH: u16 = 10;
pub(crate) const GRID_HEIGHT: u16 = 10;

// Cell
pub(crate) const CELL_WIDTH: f32 = WIDTH / GRID_WIDTH as f32;
pub(crate) const CELL_HEIGHT: f32 = HEIGHT / GRID_HEIGHT as f32;

// Player
pub(crate) const PLAYER_WIDTH: f32 = CELL_WIDTH;
pub(crate) const PLAYER_HEIGHT: f32 = CELL_HEIGHT;
pub(crate) const PLAYER_COLOR: Color = Color::hsla(120., 0.5, 0.5, 1.);
pub(crate) const PLAYER_X: i32 = 0;
pub(crate) const PLAYER_Y: i32 = 0;
pub(crate) const PLAYER_Z: i32 = 1;

// Snake tail
pub(crate) const TAIL_WIDTH: f32 = CELL_WIDTH;
pub(crate) const TAIL_HEIGHT: f32 = CELL_HEIGHT;

// Food
pub(crate) const FOOD_WIDTH: f32 = CELL_WIDTH;
pub(crate) const FOOD_HEIGHT: f32 = CELL_HEIGHT;
pub(crate) const MAX_FOOD_COUNT: usize = 10;
pub(crate) const FOOD_COLOR_RANGE: Range<u16> = 240..300;
