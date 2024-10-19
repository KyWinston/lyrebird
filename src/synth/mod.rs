use bevy::prelude::*;
use events::PlayTone;
use glicol_engine::GlicolPlugin;
use resources::{Instrument, InstrumentList};
use systems::{add_instruments, hh_808_mk, read_signal, unison};

pub mod components;
pub mod events;
pub mod glicol_engine;
pub mod resources;
pub mod systems;
pub struct SynthPlugin;

impl Plugin for SynthPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(GlicolPlugin)
            .insert_resource(InstrumentList(vec![
                Instrument {
                    name: "hi_hat".to_string(),
                    abbr: "hh".to_string(),
                    assigned_code: 127,
                    properties: vec![],
                    sound_env: hh_808_mk() + concat!(
                        "~evp: ~beat >> envperc 0.001 0.1\n",
                        "~dist: sin 5000 \n",
                        "~hi_hat: mix ~t.. >> hpf 4000 1.0 >> lpf 5000 0.2 >> mul ~dist >> mul ~evp >> hpf 10000 1.0"
                    )
                },
             
                Instrument {
                    name: "bass".to_string(),
                    abbr: "bg".to_string(),
                    assigned_code: 127,
                    properties: vec![],
                    sound_env: unison("bg".to_string(), 8, 0.2)
                        + concat!(
                            "\n~bg_pitch: ~bg_melody >> mul 50.0 \n",
                            "~bg_evp: ~bg_melody >> envperc 0.001 0.2\n",
                            "~bass: saw ~bg_pitch >> lpf 700 0.5 >> mul ~bg_evp"
                        ),
                },
                Instrument {
                    name: "bass_drum".to_string(),
                    abbr: "bd".to_string(),
                    assigned_code: 127,
                    properties: vec![],
                    sound_env: concat!(
                            "~bd_env: ~bd_beat >> envperc 0.001 0.3\n",
                            "~bd_envb: ~bd_beat >> envperc 0.001 0.3 >> mul 200.0",
                            "~bass_drum: sin ~bd_envb >> mul ~bd_env >> mul 1.0"
                        ).to_string()
                }
            ]))
            .add_event::<PlayTone>()
            .add_systems(Startup, add_instruments)
            .add_systems(Update, read_signal);
    }
}
