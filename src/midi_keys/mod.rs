use bevy::prelude::*;
#[cfg(feature="debug")]
use bevy_midi::{input::MidiInputPlugin, output::MidiOutputPlugin};
#[cfg(feature="debug")]
use systems::{connect, disconnect, refresh_ports, show_connection, show_last_message, show_ports};

pub mod components;
pub mod systems;
pub mod resources;

pub struct MidiKeysPlugin;

impl Plugin for MidiKeysPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(feature="debug")]
        app.add_plugins((MidiInputPlugin, MidiOutputPlugin))
            .add_systems(
                Update,
                (
                    refresh_ports,
                    connect,
                    disconnect,
                    show_ports,
                    show_connection,
                    show_last_message,
                ),
            );
    }
}
