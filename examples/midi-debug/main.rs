#[cfg(feature = "debug")]
use bevy::log::{Level, LogPlugin};
#[cfg(feature = "debug")]
use bevy::prelude::*;
#[cfg(feature = "debug")]
use bevy_midi::{input::MidiInputSettings, output::MidiOutputSettings};
#[cfg(feature = "debug")]
use lyrebird::LyrebirdPlugin;
#[cfg(feature = "debug")]
use systems::setup;

pub mod systems;

fn main() {
    #[cfg(feature = "debug")]
    App::new()
        .insert_resource(MidiInputSettings {
            port_name: "input",
            client_name: "input",
            ..default()
        })
        .insert_resource(MidiOutputSettings {
            port_name: "output",
        })
        .add_plugins((
            DefaultPlugins
                .set(LogPlugin {
                    level: Level::WARN,
                    filter: "bevy_midi=debug".to_string(),
                    ..default()
                })
                .build(),
            LyrebirdPlugin,
        ))
        .add_systems(Startup, setup)
        .run();
}
