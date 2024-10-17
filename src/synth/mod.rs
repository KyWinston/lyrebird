use bevy::prelude::*;
use events::PlayTone;
use glicol_engine::GlicolPlugin;
use resources::{Instrument, InstrumentList, MidiGraph};
use systems::{add_hi_hat, hh_808_mk, load_instrument, read_signal};

pub mod components;
pub mod events;
pub mod resources;
pub mod systems;
pub mod glicol_engine;
pub struct SynthPlugin;

impl Plugin for SynthPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(GlicolPlugin)
            .insert_resource(InstrumentList(vec![Instrument {
                name: "hi_hat".to_string(),
                abbr: "hh".to_string(),
                assigned_code: 127,
                sound_env: hh_808_mk()
                    + " 
                ~evp: ~beat >> envperc 0.001 0.1 
                ~dist: sin 500 >> mul 5.0
                ~hi_hat: mix ~t.. >> hpf 7000 2.0 >> lpf 7000 0.01 >> mul ~dist >> mul ~evp >> hpf 7000 1.0
            ",
            }]))
            .insert_resource(MidiGraph::default())
            .add_event::<PlayTone>()
            .add_systems(Startup, add_hi_hat)
            .add_systems(Update, read_signal)
            .observe(load_instrument);
    }
}
