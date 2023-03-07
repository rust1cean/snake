use crate::{
    config::{
        PLAYER_COLOR, PLAYER_HEIGHT, PLAYER_WIDTH, PLAYER_X, PLAYER_Y, STEP_TIME, STEP_X, STEP_Y,
    },
    grid::ToTranslation,
    not_spawned,
};
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

#[derive(Resource)]
pub struct PlayerMoveTimer(Timer);

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
        app.insert_resource(PlayerMoveTimer(Timer::from_seconds(
            STEP_TIME,
            TimerMode::Repeating,
        )))
        .add_system(Player::spawn.run_if(not_spawned::<Player>))
        .add_system(Player::moving)
        .add_system(Player::controls);
    }
}

#[derive(Component)]
pub struct Player;

impl Player {
    pub fn spawn(
        mut cmd: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<ColorMaterial>>,
    ) {
        let player: MaterialMesh2dBundle<ColorMaterial> = MaterialMesh2dBundle {
            transform: Transform {
                translation: Vec3::new(PLAYER_X, PLAYER_Y, 1.),
                scale: Vec3::new(PLAYER_WIDTH, PLAYER_HEIGHT, 1.),
                ..default()
            },
            mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
            material: materials.add(ColorMaterial::from(PLAYER_COLOR)),
            ..default()
        };

        cmd.spawn(player)
            .insert(Player)
            .insert(ToTranslation)
            .insert(Direction::None);
    }

    pub fn moving(
        time: Res<Time>,
        mut timer: ResMut<PlayerMoveTimer>,
        mut query: Query<(&Direction, &mut Transform), With<Player>>,
    ) {
        if timer.0.tick(time.delta()).just_finished() {
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
}
