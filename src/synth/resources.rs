use bevy::prelude::*;

use super::glicol_engine::GlicolEngine;

#[derive(Resource)]
pub struct Vol(pub f32);

#[derive(Resource)]
pub struct MidiGraph {
    pub tracks: Vec<String>,
    pub instruments: Vec<Instrument>,
}

#[derive(Resource, Clone)]
pub struct Instrument {
    pub name: String,
    pub abbr: String,
    pub assigned_code: u32,
    pub properties: Vec<String>,
    pub sound_env: String,
}

impl Instrument {
    pub fn new(name: String, abbr: String, volume: f32, sound_env: String) -> Self {
        Instrument {
            name,
            abbr,
            assigned_code: 127,
            properties: vec![],
            sound_env: format!("{} >> mul {}", sound_env, volume),
        }
    }
}

#[derive(Resource, Clone)]
pub struct InstrumentList(pub Vec<Instrument>);

impl InstrumentList {
    pub fn pick_by_name(&self, name: &str) -> Option<&Instrument> {
        self.0.iter().find(|&inst| inst.name == name)
    }
}

impl MidiGraph {
    pub fn new(instruments: Vec<Instrument>) -> Self {
        let mut mix = "".to_string();
        for inst in instruments.iter() {
            mix.push_str(&format!("~{} ", inst.name).to_string());
        }
        Self {
            tracks: vec![],
            instruments,
        }
    }
    pub fn update_graph(&self, engine: Res<GlicolEngine>) {
        let mut insts = vec![];
        for i in self.instruments.iter() {
            insts.push(i.sound_env.clone());
        }
        let output = &format!("{}\n{}", self.tracks.join("\n"), insts.join("\n"),).to_string();
        #[cfg(feature = "debug")]
        println!("{}", output);
        engine.update_with_code(output);
    }
}
