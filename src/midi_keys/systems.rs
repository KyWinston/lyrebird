use bevy::{
    color::palettes::css::{GREEN, RED},
    prelude::*,
};
use bevy_midi::input::{MidiData, MidiInput, MidiInputConnection};

use crate::soundfont::events::SFplayEvent;

use super::components::{Instructions, KEY_PORT_MAP};

pub fn refresh_ports(keys: Res<ButtonInput<KeyCode>>, input: Res<MidiInput>) {
    if keys.just_pressed(KeyCode::KeyR) {
        input.refresh_ports();
    }
}

pub fn connect(keys: Res<ButtonInput<KeyCode>>, input: Res<MidiInput>) {
    for (keycode, index) in &KEY_PORT_MAP {
        if keys.just_pressed(*keycode) {
            if let Some((_, port)) = input.ports().get(*index) {
                input.connect(port.clone());
            }
        }
    }
}

pub fn disconnect(keys: Res<ButtonInput<KeyCode>>, input: Res<MidiInput>) {
    if keys.just_pressed(KeyCode::Escape) {
        input.disconnect();
    }
}

pub fn show_ports(input: Res<MidiInput>, mut instructions: Query<&mut Text, With<Instructions>>) {
    if input.is_changed() {
        let text_section = &mut instructions.single_mut().sections[1];
        text_section.value = "Available input ports:\n\n".to_string();
        for (i, (name, _)) in input.ports().iter().enumerate() {
            text_section
                .value
                .push_str(format!("Port {:?}: {:?}\n", i, name).as_str());
        }
    }
}

pub fn show_connection(
    connection: Res<MidiInputConnection>,
    mut instructions: Query<&mut Text, With<Instructions>>,
) {
    if connection.is_changed() {
        let text_section = &mut instructions.single_mut().sections[2];
        if connection.is_connected() {
            text_section.value = "Connected\n".to_string();
            text_section.style.color = GREEN.into();
        } else {
            text_section.value = "Disconnected\n".to_string();
            text_section.style.color = RED.into();
        }
    }
}

pub fn show_last_message(
    mut midi_data: EventReader<MidiData>,
    mut instructions: Query<&mut Text, With<Instructions>>,
    mut midi_ev: EventWriter<SFplayEvent>,
) {
    for data in midi_data.read() {
        let text_section = &mut instructions.single_mut().sections[3];
        text_section.value = format!(
            "Last Message: {} - {:?}",
            if data.message.is_note_on() {
                "NoteOn"
            } else if data.message.is_note_off() {
                "NoteOff"
            } else {
                "Other"
            },
            data.message.msg
        );
        if data.message.is_note_on() {
            let offset = data.message.msg[1] as f32 - 57.0;
            let spacing = offset / 12.0;
            midi_ev.send(SFplayEvent(
                440.0 * 2.0_f32.powf(spacing),
                "tutorial".to_string(),
            ));
        }
    }
}