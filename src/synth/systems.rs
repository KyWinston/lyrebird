use bevy::prelude::*;

use crate::music::systems::mute_instrument;

use super::{
    events::PlaySequence,
    glicol_engine::GlicolEngine,
    resources::{Instrument, InstrumentList, MidiGraph},
};

pub fn read_signal(mut tone: EventReader<PlaySequence>, mut midi: ResMut<MidiGraph>) {
    for ev in tone.read() {
        play_sequence(&mut midi, ev.0.clone());
    }
}

pub fn init_instrument_list(mut commands: Commands) {
    commands.insert_resource(InstrumentList(vec![
        Instrument::new(
            "hi_hat".to_string(),
            "hh".to_string(),
            0.5,
            "~hh_env: ~t1_hh_beat >> envperc 0.001 0.1\n~hh_dist: sin 5000 >> mul 0.3\nhi_hat: noiz 42.0 >> mul ~hh_env >> lpf 7000 0.5 >> hpf 7000 0.5 >> mul ~hh_dist".to_string()
        ),
        Instrument::new(
            "piano".to_string(),
            "pp".to_string(),
            0.5,
            format!("~pp_pitch: ~t2_pp_melody >> mul {}\n{}\n~pp_mix: mix ~pp_p..\n~pp_evp: ~t2_pp_melody >> envperc 0.001 0.3\npiano: saw ~pp_mix >> mul ~pp_evp >> lpf 700 5.0 >> plate 0.01", 2.0_f32.powf((60.0 - 69.0) / 12.0) * 440.0, unison("pp".to_string(), 4, 0.5)).to_string()
        ),
        Instrument::new(
            "bass".to_string(),
            "bg".to_string(),
            0.5,
            format!(
                "{}\n~bg_pitch: ~t3_bg_melody >> mul 50.0\n~bg_evp: ~t3_bg_melody >> envperc 0.001 0.2\nbass: saw ~bg_pitch >> lpf 700 0.5 >> mul ~bg_evp",
                unison("bg".to_string(), 8, 0.2)
            )
            .to_string(),
        ),
        Instrument::new(
            "bass_drum".to_string(),
            "bd".to_string(),
            0.8,
            concat!(
                "~bd_env: ~t4_bd_beat >> envperc 0.001 0.3\n",
                "~bd_envb: ~t4_bd_beat >> envperc 0.001 0.3 >> mul 200.0\n",
                "bass_drum: sin ~bd_envb >> mul ~bd_env"
            )
            .to_string(),
        ),
        Instrument::new(
            "snare_drum".to_string(),
            "sd".to_string(),
            1.0,
            "~sd_env: ~t5_sd_beat >> envperc 0.001 0.07\n~sd_envb: ~t5_sd_beat >> envperc 0.001 0.1 >> mul 200.0\n~sd_drum: sin ~sd_envb >> lpf 500 0.5\n~sd_snare: noiz 40.0 >> mul ~sd_env >> mul 0.2\nsnare_drum: mix ~sd_drum ~sd_snare"
            .to_string(),
        ),
    ]));
}

pub fn add_instruments(
    mut commands: Commands,
    inst_list: Res<InstrumentList>,
    engine: ResMut<GlicolEngine>,
) {
    engine.set_livecoding(true);
    // engine.set_bpm(bpm);
    commands.insert_resource(MidiGraph::new(inst_list.0.clone()));
}

fn play_sequence(midi: &mut ResMut<MidiGraph>, id: Option<String>) {
    if let Some(id) = id {
        mute_instrument(midi, id);
    } else {
        for inst in midi.instruments.clone().into_iter() {
            mute_instrument(midi, inst.name);
        }
    }
}

//Moritz Klein hi-hat variation:mul 0.2 https://www.youtube.com/watch?v=zbBY7JL9nnQ&t=1424s
pub fn hh_808_mk() -> String {
    concat!(
        "~s1: squ 120.0\n",
        "~s2: squ 150.0\n",
        "~s3: squ 180.0\n",
        "~s4: squ 219.0\n",
        "~s5: squ 240.0\n",
        "~s6: squ 261.0\n",
    )
    .to_string()
}

pub fn unison(abbr: String, voices: u8, detune: f32) -> String {
    let mut notes = vec![];
    for v in 1..voices {
        if v == 1 {
            notes.push(format!("~{abbr}_p{v}: ~{abbr}_pitch >> mul {detune}"))
        } else {
            let uv = v - 1;
            notes.push(format!("~{abbr}_p{v}: ~{abbr}_p{uv} >> mul {detune}"))
        }
    }
    notes.join("\n")
}

pub fn change_graph(engine: Res<GlicolEngine>, midi: ResMut<MidiGraph>) {
    midi.update_graph(engine);
}
