use bevy::prelude::*;

use super::{
    components::InstrumentSource, events::PlayTone, glicol_engine::GlicolEngine, resources::{InstrumentList, MidiGraph}
};

pub fn read_signal(
    mut tone: EventReader<PlayTone>,
    engine: Res<GlicolEngine>,
    midi: ResMut<MidiGraph>,
) {
    for ev in tone.read() {
        if ev.0[0] == 145 {
            play_sequence(&engine, &midi);
        }
    }
}

pub fn add_hi_hat(mut commands: Commands) {
    commands.spawn(InstrumentSource {
        name: "hi_hat".to_string(),
        abbr: "hh".to_string(),
    });
}

pub fn load_instrument(
    trigger: Trigger<OnAdd, InstrumentSource>,
    inst_q: Query<&InstrumentSource, Added<InstrumentSource>>,
    mut midi: ResMut<MidiGraph>,
    inst_list: Res<InstrumentList>,
) {
    if let Ok(inst) = inst_q.get(trigger.entity()) {
        for instrument in midi.instruments.clone().into_iter() {
            if instrument.abbr == inst.abbr {
                return;
            }
        }
        if let Some(chosen_inst) = inst_list.pick_by_name(&inst.name) {
            midi.instruments.push(chosen_inst.clone());
        }
    }
}

fn play_sequence(engine: &Res<GlicolEngine>, midi: &ResMut<MidiGraph>) {
    let output = format!("{} {}", midi.tracks[0], midi.mixer);
    engine.update_with_code(&output);
}

pub fn midi_to_hz(signal: u32) -> f32 {
    440.0 * f32::powf(2.0, (signal as f32 - 69.0) / 12.0)
}

//Moritz Klein hi-hat variation: https://www.youtube.com/watch?v=zbBY7JL9nnQ&t=1424s
pub fn hh_808_mk() -> String {
    concat!(
        "~t1: squ 120",
        "~t2: squ 150",
        "~t3: squ 180",
        "~t4: squ 219",
        "~t5: squ 240",
        "~t6: squ 261",
        ">> mul 0.5",
    )
    .to_string()
}
