use bevy::prelude::*;
use bevy_yarnspinner::{
    events::{DialogueCompleteEvent, PresentLineEvent, PresentOptionsEvent},
    prelude::DialogueRunner,
};

use crate::dialogue_view::{
    options_selection::resource::OptionSelection,
    setup::components::{DialogueContinueNode, DialogueNameNode, UiRootNode},
    typewriter::resources::Typewriter,
};

use super::SpeakerChangeEvent;

pub fn show_dialog(mut visibility: Query<&mut Visibility, With<UiRootNode>>) {
    *visibility.single_mut() = Visibility::Inherited;
}

pub fn hide_dialog(
    mut root_visibility: Query<&mut Visibility, With<UiRootNode>>,
    mut dialogue_complete_events: EventReader<DialogueCompleteEvent>,
) {
    if !dialogue_complete_events.is_empty() {
        *root_visibility.single_mut() = Visibility::Hidden;
        dialogue_complete_events.clear();
    }
}

pub fn present_line(
    mut line_events: EventReader<PresentLineEvent>,
    mut speaker_change_events: EventWriter<SpeakerChangeEvent>,
    mut typewriter: ResMut<Typewriter>,
    mut name_node: Query<&mut Text, With<DialogueNameNode>>,
) {
    for event in line_events.read() {
        let name = if let Some(name) = event.line.character_name() {
            speaker_change_events.send(SpeakerChangeEvent {
                character_name: name.to_string(),
                speaking: true,
            });
            name.to_string()
        } else {
            String::new()
        };
        name_node.single_mut().sections[0].value = name;
        typewriter.set_line(&event.line);
    }
}

pub fn present_options(mut commands: Commands, mut events: EventReader<PresentOptionsEvent>) {
    for event in events.read() {
        let option_selection = OptionSelection::from_option_set(&event.options);
        commands.insert_resource(option_selection);
    }
}

#[allow(clippy::too_many_arguments)]
pub fn continue_dialogue(
    keys: Res<ButtonInput<KeyCode>>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    touches: Res<Touches>,
    mut dialogue_runners: Query<&mut DialogueRunner>,
    mut typewriter: ResMut<Typewriter>,
    option_selection: Option<Res<OptionSelection>>,
    mut root_visibility: Query<&mut Visibility, With<UiRootNode>>,
    mut continue_visibility: Query<
        &mut Visibility,
        (With<DialogueContinueNode>, Without<UiRootNode>),
    >,
) {
    let explicit_continue = keys.just_pressed(KeyCode::Space)
        || keys.just_pressed(KeyCode::Enter)
        || mouse_buttons.just_pressed(MouseButton::Left)
        || touches.any_just_pressed();
    if explicit_continue && !typewriter.is_finished() {
        typewriter.fast_forward();
        return;
    }
    if (explicit_continue || typewriter.last_before_options) && option_selection.is_none() {
        for mut dialogue_runner in dialogue_runners.iter_mut() {
            if !dialogue_runner.is_waiting_for_option_selection() && dialogue_runner.is_running() {
                dialogue_runner.continue_in_next_update();
                *root_visibility.single_mut() = Visibility::Hidden;
                *continue_visibility.single_mut() = Visibility::Hidden;
            }
        }
    }
}
