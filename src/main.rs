use bevy::prelude::*;
use snake::{Game, WindowSetup};

// TODO:
// * Coordiante system
// * Spawn food there where no snake
// * Debug grid layout

fn main() {
    App::new().add_plugin(WindowSetup).add_plugin(Game).run();
}
