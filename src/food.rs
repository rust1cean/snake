use crate::{
    config::{
        COUNT_CELLS_HEIGHT, COUNT_CELLS_WIDTH, FOOD_COLOR_RANGE, FOOD_HEIGHT, FOOD_WIDTH,
        MAX_FOOD_COUNT,
    },
    ground::{CoordSystem, Position},
    MaxEntities,
};
use bevy::prelude::*;
use rand::Rng;

pub struct FoodPlugin;

impl Plugin for FoodPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(Food::spawn.run_if(MaxFoodCount::is_more_than::<Food>));
    }
}

#[derive(Component)]
pub struct Obstacle;

#[derive(Component)]
pub struct Food;

impl Food {
    pub fn spawn(mut obstacles: Query<&Position, With<Obstacle>>, mut cmd: Commands) {
        let mut spawn_food = |position: Position| -> () {
            let color: Color = Self::food_color();
            let mesh: SpriteBundle = SpriteBundle {
                sprite: Sprite {
                    color,
                    custom_size: Some(Vec2::new(FOOD_WIDTH, FOOD_HEIGHT)),
                    ..default()
                },
                ..default()
            };

            cmd.spawn((mesh, position, Food, Obstacle));
        };

        let mut no_obstacles_for = |pos: &Position| -> bool {
            for obs in &mut obstacles {
                if CoordSystem::in_one_position(&obs, &pos) {
                    return false;
                }
            }
            true
        };

        let position: Position = Self::random_position();

        if no_obstacles_for(&position) {
            spawn_food(position);
        }
    }

    pub fn food_color() -> Color {
        let random = rand::thread_rng().gen_range(FOOD_COLOR_RANGE);
        Color::hsla(random as f32, 0.5, 0.5, 1.)
    }

    pub fn random_position() -> Position {
        let mut rng = rand::thread_rng();

        let x: i32 = rng.gen_range(0..COUNT_CELLS_WIDTH as i32);
        let y: i32 = rng.gen_range(0..COUNT_CELLS_HEIGHT as i32);
        let z: i32 = 0;

        Position { x, y, z }
    }
}

pub struct MaxFoodCount;

impl MaxEntities for MaxFoodCount {
    const COUNT: usize = MAX_FOOD_COUNT;
}
