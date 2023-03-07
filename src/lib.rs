pub mod config;
pub mod grid;
pub mod player;

use bevy::{core_pipeline::clear_color::ClearColorConfig, prelude::*, window::WindowResolution};
use config::{HEIGHT, TITLE, WIDTH};
use grid::GridPlugin;
use player::PlayerPlugin;

pub struct WindowSetup;

impl Plugin for WindowSetup {
    fn build(&self, app: &mut App) {
        app.add_state::<State>()
            .add_startup_system(Self::setup_camera)
            .add_plugins(DefaultPlugins.set(self.window_plugin()));
    }
}

impl WindowSetup {
    pub fn setup_camera(mut cmd: Commands) {
        cmd.spawn(Camera2dBundle {
            camera_2d: Camera2d {
                clear_color: ClearColorConfig::Custom(Color::rgb(0., 0., 0.)),
                ..default()
            },
            ..default()
        });
    }

    pub fn window_plugin(&self) -> WindowPlugin {
        WindowPlugin {
            primary_window: Some(Window {
                title: TITLE.to_string(),
                resizable: false,
                resolution: WindowResolution::new(WIDTH, HEIGHT),
                ..default()
            }),
            ..default()
        }
    }
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(PlayerPlugin).add_plugin(GridPlugin);
    }
}

impl GamePlugin {}

#[derive(Hash, Clone, States, PartialEq, Eq, Debug, Default)]
pub enum State {
    #[default]
    MainMenu,
}

/// Checks for the absence of an entity
///
/// Example to use:
/// app.add_system(spawn.run_if(not_spawned::<Player>));
pub fn not_spawned<T: Component>(entity: Query<&T>) -> bool {
    entity.is_empty()
}

/// Checks for the presence of an entity
///
/// Example to use:
/// app.add_system(go_to_home.run_if(spawned::<Player>));
pub fn spawned<T: Component>(entity: Query<&T>) -> bool {
    !entity.is_empty()
}