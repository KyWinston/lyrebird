use bevy::prelude::*;
use systems::setup;

pub mod components;
pub mod systems;
pub mod style;

pub fn ui_setup_plugin(app: &mut App) {
    app.add_systems(Startup, setup);
}
