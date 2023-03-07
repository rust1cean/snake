use bevy::prelude::Color;

// Window
pub(crate) const TITLE: &str = "Snake";
pub(crate) const WIDTH: f32 = 300.;
pub(crate) const HEIGHT: f32 = 300.;
pub(crate) const HALF_WIDTH: f32 = WIDTH / 2.;
pub(crate) const HALF_HEIGHT: f32 = HEIGHT / 2.;

// Grid
pub(crate) const COUNT_CELLS_WIDTH: usize = 60;
pub(crate) const COUNT_CELLS_HEIGHT: usize = 60;

// Cell
pub(crate) const CELL_WIDTH: f32 = WIDTH / COUNT_CELLS_WIDTH as f32;
pub(crate) const CELL_HEIGHT: f32 = HEIGHT / COUNT_CELLS_HEIGHT as f32;

// Player
pub(crate) const PLAYER_WIDTH: f32 = CELL_WIDTH;
pub(crate) const PLAYER_HEIGHT: f32 = CELL_HEIGHT;
pub(crate) const PLAYER_X: f32 = 0.;
pub(crate) const PLAYER_Y: f32 = 0.;
pub(crate) const PLAYER_COLOR: Color = Color::rgb(0., 0.6, 0.);
pub(crate) const STEP_X: f32 = CELL_WIDTH;
pub(crate) const STEP_Y: f32 = CELL_HEIGHT;
pub(crate) const STEP_TIME: f32 = 0.1;
