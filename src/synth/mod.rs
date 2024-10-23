use bevy::prelude::*;
use events::{PlaySequence, PlayTone};
use glicol_engine::GlicolPlugin;
use resources::MidiGraph;
use systems::{add_instruments, change_graph, init_instrument_list, read_signal};

pub mod components;
pub mod events;
pub mod glicol_engine;
pub mod resources;
pub mod systems;
pub struct SynthPlugin;

impl Plugin for SynthPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(GlicolPlugin)
            .add_systems(PreStartup, init_instrument_list)
            .add_event::<PlayTone>()
            .add_event::<PlaySequence>()
            .add_systems(Startup, add_instruments)
            .add_systems(
                Update,
                (
                    read_signal,
                    change_graph.run_if(resource_changed::<MidiGraph>),
                ),
            );
    }
}
