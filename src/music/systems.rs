use crate::synth::resources::MidiGraph;
use bevy::prelude::*;

pub fn add_track(mut midi: MidiGraph, track: String) {
    let tracks_len = midi.tracks.len();
    midi.tracks.push(format!("~t{}: {track}", tracks_len));
}

pub fn remove_track(mut midi: MidiGraph, track: usize) {
    midi.tracks.remove(track);
}

pub fn mute_instrument(midi: &mut ResMut<MidiGraph>, instrument: String) {
    let inst = midi
        .instruments
        .iter_mut()
        .find(|inst| inst.name == instrument);
    if let Some(inst) = inst {
        let inst_name = format!("//{}", inst.name);
        if !inst.sound_env.contains("//") {
            inst.sound_env = inst.sound_env.replace(&inst.name, &inst_name);
        } else {
            unmute_instrument(midi, instrument);
        }
    }
}

pub fn unmute_instrument(midi: &mut ResMut<MidiGraph>, instrument: String) {
    let inst = midi
        .instruments
        .iter_mut()
        .find(|inst| inst.name == instrument);
    if let Some(inst) = inst {
        let inst_name = format!("//{}", inst.name);
        if inst.sound_env.contains("//") {
            inst.sound_env = inst.sound_env.replace(&inst_name, &inst.name);
        } else {
            mute_instrument(midi, instrument);
        }
    }
}
