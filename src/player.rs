use crate::{
    config::{
        HALF_HEIGHT, HALF_WIDTH, PLAYER_COLOR, PLAYER_HEIGHT, PLAYER_WIDTH, PLAYER_X, PLAYER_Y,
        PLAYER_Z, STEP_TIME, TAIL_HEIGHT, TAIL_WIDTH,
    },
    food::{Food, Obstacle},
    ground::{CoordSystem, Position},
    not_spawned,
};
use bevy::prelude::*;

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
    pub fn spawn(mut cmd: Commands) {
        let mesh: SpriteBundle = SpriteBundle {
            sprite: Sprite {
                color: PLAYER_COLOR,
                custom_size: Some(Vec2::new(PLAYER_WIDTH, PLAYER_HEIGHT)),
                ..default()
            },
            ..default()
        };

        cmd.spawn((
            mesh,
            Player,
            SnakeEnd,
            Obstacle,
            Position::new(PLAYER_X, PLAYER_Y, PLAYER_Z),
            Direction::None,
        ));
    }

    pub fn moving(mut query: Query<(&Direction, &mut Position), With<Player>>) {
        if let Ok((direction, mut position)) = query.get_single_mut() {
            match *direction {
                Direction::Left => position.x -= 1,
                Direction::Right => position.x += 1,
                Direction::Up => position.y += 1,
                Direction::Down => position.y -= 1,
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
        player: Query<&Position, With<Self>>,
        food: Query<(&Position, Entity), With<Food>>,
    ) {
        if let Ok(player) = player.get_single() {
            let is_it_eaten = |food: (&Position, Entity)| -> () {
                let (food, entity): (&Position, Entity) = food;

                if CoordSystem::in_one_position(&player, food) {
                    cmd.entity(entity).despawn();

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
        query: Query<(Entity, &Position), With<SnakeEnd>>,
    ) {
        let snake_has_grown: bool = growth_reader.iter().next().is_some();

        if snake_has_grown {
            if let Ok((segment_end, position)) = query.get_single() {
                let mesh: SpriteBundle = SpriteBundle {
                    sprite: Sprite {
                        color: PLAYER_COLOR,
                        custom_size: Some(Vec2::new(TAIL_WIDTH, TAIL_HEIGHT)),
                        ..default()
                    },
                    ..default()
                };

                // Add segment
                cmd.spawn((
                    mesh,
                    Obstacle,
                    TailSegment,
                    Position::new(position.x, position.y, position.z),
                    SnakeEnd,
                ));

                // Remove previous segment end
                cmd.entity(segment_end).remove::<SnakeEnd>();
            }
        }
    }

    pub fn tail_move(
        player: Query<&Position, With<Player>>,
        mut tail: Query<&mut Position, (With<TailSegment>, Without<Player>)>,
    ) {
        if let Ok(player) = player.get_single() {
            let mut prev: Position = player.clone();

            tail.for_each_mut(|mut segment| {
                (*segment, prev) = (prev, *segment);
            });
        }
    }
}
