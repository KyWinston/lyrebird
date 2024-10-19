use bevy::prelude::Resource;

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
    pub properties: Vec<String>,
    pub sound_env: String,
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
            tracks: vec![
                "~beat: speed 6.0 >> seq 1".to_string(),
                "~bg_melody: speed 1.0 >> seq 60 _ 70 60 60_70 70_70".to_string(),
                "~bd_beat: seq 60 _ 60 60 _ 60_60".to_string(),
            ],
            instruments,
            mixer: format!("o: mix {mix} >> mul 1.0").to_string(),
        }
    }
}
