use bevy::prelude::Resource;

use super::systems::hh_808_mk;

#[derive(Resource)]
pub struct Vol(pub f32);

#[derive(Resource)]
pub struct MidiGraph {
    pub tracks: Vec<String>,
    pub instruments: Vec<Instrument>,
    pub mixer: String,
}

#[derive(Resource, Clone)]
pub struct Instrument {
    pub name: String,
    pub abbr: String,
    pub assigned_code: u32,
    pub sound_env: String,
}

#[derive(Resource, Clone)]
pub struct InstrumentList(pub Vec<Instrument>);

impl InstrumentList {
    pub fn pick_by_name(&self, name: &str) -> Option<&Instrument> {
        self.0.iter().find(|&inst| inst.name == name)
    }
}

impl Default for MidiGraph {
    fn default() -> Self {
        Self {
            tracks: vec!["~beat: imp 1.0 >> ~hi_hat".to_string()],
            instruments: vec![Instrument {
                name: "hi_hat".to_string(),
                abbr: "hh".to_string(),
                assigned_code: 127,
                sound_env: hh_808_mk()
                    + " 
                ~evp: ~beat >> envperc 0.001 0.1 
                ~dist: sin 500 >> mul 5.0
                ~hi_hat: mix ~t.. >> hpf 7000 2.0 >> lpf 7000 0.01 >> mul ~dist >> mul ~evp >> hpf 7000 1.0
            ",
            }],
            mixer: "o: ~t1".to_string(),
        }
    }
}
