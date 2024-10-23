#[cfg(feature = "midi")]
use bevy::prelude::*;
#[cfg(feature = "midi")]
use bevy_midi::{input::MidiInputPlugin, output::MidiOutputPlugin};

#[cfg(feature = "midi")]
use systems::{connect, disconnect, refresh_ports, show_connection, show_last_message, show_ports};

pub mod components;
pub mod resources;
pub mod systems;

#[cfg(feature = "midi")]
pub struct MidiKeysPlugin;
#[cfg(feature = "midi")]
impl Plugin for MidiKeysPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((MidiInputPlugin, MidiOutputPlugin));
        #[cfg(feature = "midi")]
        app.add_systems(
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
