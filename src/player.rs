use crate::{
    config::{
        HALF_HEIGHT, HALF_WIDTH, PLAYER_COLOR, PLAYER_HEIGHT, PLAYER_WIDTH, PLAYER_X, PLAYER_Y,
        PLAYER_Z, TAIL_HEIGHT, TAIL_WIDTH,
    },
    food::{Food, Obstacle},
    ground::{CoordSystem, Position},
    not_spawned,
};
use bevy::prelude::*;

pub struct PlayerGrowthEvent;

#[derive(Component)]
pub struct Tail;

#[derive(Component)]
pub struct TailSegment;

#[derive(Component)]
pub struct SnakeHead;

#[derive(Component)]
pub struct SnakeEnd;

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
        app.add_systems((
            Player::spawn.run_if(not_spawned::<Player>),
            Player::growth,
            Player::collision_with_tail.in_schedule(CoreSchedule::FixedUpdate),
            Player::moving.in_schedule(CoreSchedule::FixedUpdate),
            Player::out_bounds.in_schedule(CoreSchedule::FixedUpdate),
            Player::eat.in_schedule(CoreSchedule::FixedUpdate),
            Player::controls,
        ))
        .add_event::<PlayerGrowthEvent>();
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
            SnakeHead,
            SnakeEnd,
            Obstacle,
            Position::new(PLAYER_X, PLAYER_Y, PLAYER_Z),
            Direction::None,
        ));
    }

    pub fn moving(
        mut head: Query<(&Direction, &mut Position), With<Player>>,
        mut tail: Query<&mut Position, (With<TailSegment>, Without<Player>)>,
    ) {
        if let Ok((direction, mut position)) = head.get_single_mut() {
            // Move tail
            {
                let mut prev: Position = position.clone();

                tail.for_each_mut(|mut segment| {
                    (*segment, prev) = (prev, *segment);
                });
            }

            // Move head
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

    pub fn collision_with_tail(
        head: Query<&Position, With<SnakeHead>>,
        tail: Query<&Position, With<TailSegment>>,
    ) {
        if let Ok(head) = head.get_single() {
            tail.for_each(|tail| {
                if CoordSystem::in_one_position(&head, &tail) {
                    dbg!("Collision with tail!");
                }
            });
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
}
