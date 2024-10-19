use bevy::prelude::*;

use super::{
    events::PlayTone,
    glicol_engine::GlicolEngine,
    resources::{InstrumentList, MidiGraph},
};

pub fn read_signal(
    mut tone: EventReader<PlayTone>,
    engine: Res<GlicolEngine>,
    midi: ResMut<MidiGraph>,
) {
    for ev in tone.read() {
        if ev.0[0] == 145 {
            play_sequence(&engine, &midi, ev.2);
        }
    }
}

pub fn add_instruments(mut commands: Commands, inst_list: Res<InstrumentList>) {
    commands.insert_resource(MidiGraph::new(inst_list.0.clone()));
}

fn play_sequence(engine: &Res<GlicolEngine>, midi: &ResMut<MidiGraph>, id: Option<usize>) {
    let output = if id.is_none() {
        let mut insts = vec![];
        for i in midi.instruments.iter() {
            insts.push(i.sound_env.clone());
        }
        format!(
            "{}\n{}\n{}",
            midi.tracks.join("\n"),
            insts.join("\n"),
            midi.mixer
        )
    } else {
        format!(
            "{}\n{}\no: ~{} >> mul 1.0",
            midi.tracks[id.unwrap()],
            midi.instruments[id.unwrap()].sound_env,
            midi.instruments[id.unwrap()].name
        )
    };
    println!("{}", output);
    engine.update_with_code(&output);
}

pub fn midi_to_hz(signal: u32) -> f32 {
    440.0 * f32::powf(2.0, (signal as f32 - 69.0) / 12.0)
}

//Moritz Klein hi-hat variation: https://www.youtube.com/watch?v=zbBY7JL9nnQ&t=1424s
pub fn hh_808_mk() -> String {
    concat!(
        "~t1: squ 120\n",
        "~t2: squ 150\n",
        "~t3: squ 180\n",
        "~t4: squ 219\n",
        "~t5: squ 240\n",
        "~t6: squ 261\n",
    )
    .to_string()
}

pub fn unison(abbr: String, voices: u8, detune: f32) -> String {
    let mut notes = vec![];
    for v in 1..voices {
        if v == 1 {
            notes.push(format!("~{abbr}_p{v}: ~{abbr}_pitch>> mul {detune}"))
        } else {
            let uv = v - 1;
            notes.push(format!("~{abbr}_p{v}: ~{abbr}_p{uv} >> mul {detune}"))
        }
    }
    notes.join("\n")
}
