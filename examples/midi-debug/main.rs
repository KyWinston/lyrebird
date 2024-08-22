use bevy::{log::{Level, LogPlugin}, prelude::*};
use bevy_midi::input::MidiInputSettings;
use parakeet::ParakeetPlugin;
use systems::setup;

pub mod systems;

fn main() {
    App::new()
        .insert_resource(MidiInputSettings {
            port_name: "input",
            client_name: "input",
            ..default()
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
