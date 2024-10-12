#[cfg(feature = "debug")]
use bevy_midi::{input::MidiInputSettings, output::MidiOutputSettings};

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
