use bevy::prelude::*;
use snake::{GamePlugin, WindowSetup};

fn main() {
    App::new()
        .add_plugin(WindowSetup)
        .add_plugin(GamePlugin)
        .run();
}
