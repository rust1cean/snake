use crate::{
    config::{
        HALF_HEIGHT, HALF_WIDTH, PLAYER_COLOR, PLAYER_HEIGHT, PLAYER_WIDTH, PLAYER_X, PLAYER_Y,
        STEP_TIME, STEP_X, STEP_Y, TAIL_HEIGHT, TAIL_WIDTH,
    },
    food::Food,
    grid::ToTranslation,
    not_spawned,
};
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

#[derive(Component)]
pub struct TailSegment;

#[derive(Component)]
pub struct SnakeEnd;

pub struct PlayerGrowthEvent;

#[derive(Component, PartialEq, Eq)]
pub enum Direction {
    Left,
    Up,
    Right,
    Down,
    None,
}

impl Direction {
    pub fn invert(&self) -> Self {
        match self {
            Self::Left => Self::Right,
            Self::Up => Self::Down,
            Self::Right => Self::Left,
            Self::Down => Self::Up,
            Self::None => Self::None,
        }
    }
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(Player::out_bounds)
            .add_system(Player::controls)
            .add_system(Player::eat)
            .add_system(Tail::growth);

        // Events
        app.add_event::<PlayerGrowthEvent>();

        // Conditions
        app.add_system(Player::spawn.run_if(not_spawned::<Player>));

        // In schedule
        app.insert_resource(FixedTime::new_from_secs(STEP_TIME))
            .add_system(Player::moving.in_schedule(CoreSchedule::FixedUpdate))
            .add_system(Tail::tail_move.in_schedule(CoreSchedule::FixedUpdate));
    }
}

#[derive(Component, Debug)]
pub struct Player;

impl Player {
    pub fn spawn(
        mut cmd: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<ColorMaterial>>,
    ) {
        let player_mesh: MaterialMesh2dBundle<ColorMaterial> = MaterialMesh2dBundle {
            transform: Transform {
                translation: Vec3::new(PLAYER_X, PLAYER_Y, 1.),
                scale: Vec3::new(PLAYER_WIDTH, PLAYER_HEIGHT, 1.),
                ..default()
            },
            mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
            material: materials.add(ColorMaterial::from(PLAYER_COLOR)),
            ..default()
        };

        cmd.spawn(player_mesh)
            .insert(Player)
            .insert(SnakeEnd)
            .insert(ToTranslation)
            .insert(Direction::None);
    }

    pub fn moving(mut query: Query<(&Direction, &mut Transform), With<Player>>) {
        if let Ok((direction, mut transform)) = query.get_single_mut() {
            match *direction {
                Direction::Left => transform.translation.x -= STEP_X,
                Direction::Right => transform.translation.x += STEP_X,
                Direction::Up => transform.translation.y += STEP_Y,
                Direction::Down => transform.translation.y -= STEP_Y,
                Direction::None => (),
            }
        }
    }

    pub fn controls(key: Res<Input<KeyCode>>, mut query: Query<&mut Direction, With<Player>>) {
        if let Ok(mut direction) = query.get_single_mut() {
            let new_direction = if key.pressed(KeyCode::Left) || key.pressed(KeyCode::A) {
                Direction::Left
            } else if key.pressed(KeyCode::Up) || key.pressed(KeyCode::W) {
                Direction::Up
            } else if key.pressed(KeyCode::Right) || key.pressed(KeyCode::D) {
                Direction::Right
            } else if key.pressed(KeyCode::Down) || key.pressed(KeyCode::S) {
                Direction::Down
            } else {
                Direction::None
            };

            if new_direction != direction.invert() && new_direction != Direction::None {
                *direction = new_direction;
            }
        }
    }

    pub fn out_bounds(transform: Query<&Transform, With<Player>>) {
        if let Ok(transform) = transform.get_single() {
            let Transform { translation, .. } = transform;

            let out_bounds: bool = translation.x < -HALF_WIDTH
                || translation.x > HALF_WIDTH
                || translation.y < -HALF_HEIGHT
                || translation.y > HALF_HEIGHT;

            if out_bounds {
                dbg!("OUT OF BOUNDS");
            }
        }
    }

    pub fn eat(
        mut cmd: Commands,
        mut growth_writer: EventWriter<PlayerGrowthEvent>,
        player: Query<&Transform, With<Self>>,
        food: Query<(&Transform, Entity), With<Food>>,
    ) {
        if let Ok(player_transform) = player.get_single() {
            let x: f32 = player_transform.translation.x;
            let y: f32 = player_transform.translation.y;
            let half_scale_x: f32 = player_transform.scale.x / 2.;
            let half_scale_y: f32 = player_transform.scale.y / 2.;

            let player_x1: f32 = x - half_scale_x; // Tangent to the left side of the snake's head
            let player_x2: f32 = x + half_scale_x; // Tangent to the right side of the snake's head
            let player_y1: f32 = y - half_scale_y; // Tangent to the top side of the snake's head
            let player_y2: f32 = y + half_scale_y; // Tangent to the bottom side of the snake's head

            let is_it_eaten = |food: (&Transform, Entity)| -> () {
                let (food_transform, food) = food;

                let food_x: f32 = food_transform.translation.x;
                let food_y: f32 = food_transform.translation.y;

                if (food_x >= player_x1 && food_x <= player_x2)
                    && (food_y >= player_y1 && food_y <= player_y2)
                {
                    cmd.entity(food).despawn();

                    growth_writer.send(PlayerGrowthEvent);
                }
            };

            food.for_each(is_it_eaten);
        }
    }
}

#[derive(Component)]
pub struct Tail;

impl Tail {
    pub fn growth(
        mut cmd: Commands,
        mut growth_reader: EventReader<PlayerGrowthEvent>,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<ColorMaterial>>,
        query: Query<(Entity, &Transform), With<SnakeEnd>>,
    ) {
        let snake_has_grown: bool = growth_reader.iter().next().is_some();

        if snake_has_grown {
            if let Ok((segment_end, transform)) = query.get_single() {
                let segment_mesh: MaterialMesh2dBundle<ColorMaterial> = MaterialMesh2dBundle {
                    transform: Transform {
                        translation: transform.translation,
                        scale: Vec3::new(TAIL_WIDTH, TAIL_HEIGHT, 1.),
                        ..default()
                    },
                    mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
                    material: materials.add(ColorMaterial::from(PLAYER_COLOR)),
                    ..default()
                };

                // Add segment
                cmd.spawn(segment_mesh).insert(TailSegment).insert(SnakeEnd);

                // Remove previous segment end
                cmd.entity(segment_end).remove::<SnakeEnd>();
            }
        }
    }

    pub fn tail_move(
        player: Query<&Transform, With<Player>>,
        mut tail: Query<&mut Transform, (With<TailSegment>, Without<Player>)>,
    ) {
        if let Ok(player) = player.get_single() {
            let mut prev: Transform = player.clone();

            tail.for_each_mut(|mut segment| {
                (*segment, prev) = (prev, *segment);
            });
        }
    }
}
