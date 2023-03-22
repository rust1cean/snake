use bevy::prelude::*;
use snake::{Game, WindowSetup};

fn main() {
    App::new().add_plugin(WindowSetup).add_plugin(Game).run();
}
