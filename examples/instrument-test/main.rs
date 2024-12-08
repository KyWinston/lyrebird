use bevy::prelude::*;
use lyrebird::LyrebirdPlugin;
use systems::{play_instrument, setup};

pub mod systems;



fn main() {
    App::new()
        .add_plugins((DefaultPlugins, LyrebirdPlugin))
        .add_systems(Startup, setup)
        .add_systems(Update, play_instrument)
        .run();
}
