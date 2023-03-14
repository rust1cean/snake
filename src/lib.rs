pub mod config;
pub mod food;
pub mod ground;
pub mod player;

use bevy::{core_pipeline::clear_color::ClearColorConfig, prelude::*, window::WindowResolution};
use config::{HEIGHT, TITLE, WIDTH};
use food::FoodPlugin;
use ground::GroundPlugin;
use player::PlayerPlugin;

pub struct WindowSetup;

impl Plugin for WindowSetup {
    fn build(&self, app: &mut App) {
        app.add_state::<AppState>()
            .add_startup_system(Self::setup_camera)
            .add_plugins(DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Self::window()),
                ..default()
            }));
    }
}

impl WindowSetup {
    pub fn window() -> Window {
        Window {
            title: TITLE.to_string(),
            resizable: false,
            // decorations: false,
            resolution: WindowResolution::new(WIDTH, HEIGHT),
            position: WindowPosition::new(IVec2::new(0, 0)),
            ..default()
        }
    }

    pub fn setup_camera(mut cmd: Commands) {
        cmd.spawn(Camera2dBundle {
            camera_2d: Camera2d {
                clear_color: ClearColorConfig::Custom(Color::BLACK),
                ..default()
            },
            ..default()
        });
    }
}

pub struct Game;

impl Plugin for Game {
    fn build(&self, app: &mut App) {
        app.add_plugin(PlayerPlugin)
            .add_plugin(GroundPlugin)
            .add_plugin(FoodPlugin);
    }
}

impl Game {}

#[derive(Hash, Clone, States, PartialEq, Eq, Debug, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    InGame,
}

/// Checks for the absence of an entity
///
/// # Example:
/// ```rust
/// app.add_system(spawn.run_if(not_spawned::<Player>));
/// ```
pub fn not_spawned<T: Component>(entity: Query<&T>) -> bool {
    entity.is_empty()
}

/// Checks for the presence of an entity
///
/// # Example:
/// ```rust
/// app.add_system(go_to_home.run_if(spawned::<Player>));
/// ```
pub fn spawned<T: Component>(entity: Query<&T>) -> bool {
    !entity.is_empty()
}

/// Compares the maximum number of entities with the current ones
///
/// # Example:
/// ```rust
/// app.add_system(Food::spawn.run_if(MaxFoodCount::is_more_than::<Food>));
/// ```
pub trait MaxEntities {
    const COUNT: usize;

    fn is_more_than<T: Component>(entities: Query<&T>) -> bool {
        Self::COUNT > entities.iter().len()
    }
}
