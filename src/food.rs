use crate::{
    config::{CELL_HEIGHT, CELL_WIDTH, COUNT_CELLS_HEIGHT, COUNT_CELLS_WIDTH, MAX_FOOD_COUNT},
    grid::ToTranslation,
    MaxEntities,
};
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use rand::Rng;

pub struct FoodPlugin;

impl Plugin for FoodPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(Food::spawn.run_if(MaxFoodCount::is_more_than::<Food>));
    }
}

#[derive(Component)]
pub struct Food;

impl Food {
    pub fn spawn(
        mut cmd: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<ColorMaterial>>,
    ) {
        let food_mesh: MaterialMesh2dBundle<ColorMaterial> = MaterialMesh2dBundle {
            transform: Transform {
                translation: Self::random_position(),
                scale: Self::random_scale(),
                ..default()
            },
            mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
            material: materials.add(ColorMaterial::from(Self::food_color())),
            ..default()
        };

        cmd.spawn(food_mesh).insert(Food).insert(ToTranslation);
    }

    pub fn food_color() -> Color {
        let random = rand::thread_rng().gen_range(60..120);
        Color::hsla(random as f32, 100., 50., 1.)
    }

    pub fn random_position() -> Vec3 {
        let mut rng = rand::thread_rng();

        let horizontal_count = rng.gen_range(0..COUNT_CELLS_WIDTH);
        let vertical_count = rng.gen_range(0..COUNT_CELLS_HEIGHT);

        let x = (CELL_WIDTH) * horizontal_count as f32;
        let y = (CELL_HEIGHT) * vertical_count as f32;
        let z = 1.;

        Vec3::new(x, y, z)
    }

    pub fn random_scale() -> Vec3 {
        Vec3::new(CELL_WIDTH, CELL_HEIGHT, 1.)
    }
}

pub struct MaxFoodCount;

impl MaxEntities for MaxFoodCount {
    const COUNT: usize = MAX_FOOD_COUNT;
}
