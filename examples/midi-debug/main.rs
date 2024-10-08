use bevy::{
    log::{Level, LogPlugin},
    prelude::*,
};

#[cfg(feature = "debug")]
use bevy_midi::{input::MidiInputSettings, output::MidiOutputSettings};

use lyrebird::LyrebirdPlugin;
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
            ParakeetPlugin,
        ))
        .add_systems(Startup, setup)
        .run();
}
