use bevy::prelude::*;
use bevy_glicol::prelude::*;
use events::PlayTone;
use resources::MidiGraph;
use systems::{add_hi_hat, load_instrument, read_signal};

pub mod components;
pub mod events;
pub mod resources;
pub mod systems;
pub struct SynthPlugin;

impl Plugin for SynthPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(GlicolPlugin)
            .insert_resource(MidiGraph::default())
            .add_event::<PlayTone>()
            .add_systems(Startup, add_hi_hat)
            .add_systems(Update, read_signal)
            .observe(load_instrument);
    }
}
