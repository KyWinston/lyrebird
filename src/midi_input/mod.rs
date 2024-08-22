use bevy::{
    log::{Level, LogPlugin},
    prelude::*,
};
use systems::{connect, disconnect, refresh_ports, show_connection, show_last_message, show_ports};

pub mod components;
pub mod systems;

pub struct MidiInputPlugin;

impl Plugin for MidiInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MidiInputPlugin).add_systems(
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
