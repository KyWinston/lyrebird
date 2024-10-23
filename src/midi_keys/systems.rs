#[cfg(feature = "debug")]
use crate::synth::events::PlayTone;
#[cfg(feature = "debug")]
use bevy::{
    color::palettes::css::{GREEN, RED},
    prelude::*,
};

#[cfg(feature = "midi")]
use bevy_midi::input::{MidiData, MidiInput, MidiInputConnection};
#[cfg(feature = "midi")]
use super::components::{Instructions, KEY_PORT_MAP};

#[cfg(feature = "midi")]
pub fn refresh_ports(keys: Res<ButtonInput<KeyCode>>, input: Res<MidiInput>) {
    if keys.just_pressed(KeyCode::KeyR) {
        input.refresh_ports();
    }
}
#[cfg(feature = "midi")]
pub fn connect(keys: Res<ButtonInput<KeyCode>>, input: Res<MidiInput>) {
    for (keycode, index) in &KEY_PORT_MAP {
        if keys.just_pressed(*keycode) {
            if let Some((_, port)) = input.ports().get(*index) {
                input.connect(port.clone());
            }
        }
    }
}
#[cfg(feature = "midi")]
pub fn disconnect(keys: Res<ButtonInput<KeyCode>>, input: Res<MidiInput>) {
    if keys.just_pressed(KeyCode::Escape) {
        input.disconnect();
    }
}
#[cfg(feature = "midi")]
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
#[cfg(feature = "midi")]
pub fn show_connection(
    connection: Res<MidiInputConnection>,
    mut instructions: Query<&mut Text, With<Instructions>>,
    tone: EventWriter<PlayTone>,
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
#[cfg(feature = "midi")]
pub fn show_last_message(
    mut midi_data: EventReader<MidiData>,
    mut instructions: Query<&mut Text, With<Instructions>>,
    mut tone: EventWriter<PlayTone>,
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
        tone.send(PlayTone(data.message.msg, "hi_hat".to_string()));
    }
}
