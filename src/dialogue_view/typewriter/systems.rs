use crate::dialogue_view::{
    options_selection::resource::OptionSelection,
    setup::{
        components::{DialogueContinueNode, DialogueNode, UiRootNode},
        systems::create_dialog_text,
    },
    updating::events::SpeakerChangeEvent,
};
use bevy::prelude::*;

use super::{events::TypewriterFinishedEvent, resources::Typewriter};

pub fn show_continue(
    typewriter: Res<Typewriter>,
    mut visibility: Query<&mut Visibility, With<DialogueContinueNode>>,
    mut typewriter_finished_event: EventReader<TypewriterFinishedEvent>,
) {
    for _event in typewriter_finished_event.read() {
        if !typewriter.last_before_options {
            let mut visibility = visibility.single_mut();
            *visibility = Visibility::Inherited;
        }
    }
}

pub fn despawn(mut commands: Commands) {
    commands.remove_resource::<Typewriter>();
}

pub fn spawn(mut commands: Commands) {
    commands.init_resource::<Typewriter>();
}

pub fn bob_continue(
    time: Res<Time>,
    mut visibility: Query<(&Visibility, &mut Style), With<DialogueContinueNode>>,
) {
    if let Ok((visibility, mut style)) = visibility.get_single_mut() {
        if *visibility == Visibility::Hidden {
            return;
        }
        let pixels = (time.elapsed_seconds() * 3.0).sin().powi(2) * 5.0;
        style.bottom = Val::Px(pixels);
    }
}

pub fn send_finished_event(
    mut events: EventWriter<TypewriterFinishedEvent>,
    typewriter: Res<Typewriter>,
    mut last_finished: Local<bool>,
) {
    if !typewriter.is_finished() {
        *last_finished = false;
    } else if !*last_finished {
        events.send(TypewriterFinishedEvent);
        *last_finished = true;
    }
}

pub fn write_text(
    mut text: Query<&mut Text, With<DialogueNode>>,
    mut typewriter: ResMut<Typewriter>,
    option_selection: Option<Res<OptionSelection>>,
    mut speaker_change_events: EventWriter<SpeakerChangeEvent>,
    mut root_visibility: Query<&mut Visibility, With<UiRootNode>>,
) {
    let mut text = text.single_mut();
    if typewriter.last_before_options && option_selection.is_none() {
        *text = default();
        return;
    }
    if typewriter.is_finished() {
        return;
    }
    if !typewriter.last_before_options {
        *root_visibility.single_mut() = Visibility::Inherited;
    }
    typewriter.update_current_text();
    if typewriter.is_finished() {
        if let Some(name) = typewriter.character_name.as_deref() {
            speaker_change_events.send(SpeakerChangeEvent {
                character_name: name.to_string(),
                speaking: false,
            });
        }
    }

    let current_text = &typewriter.current_text;
    let rest = typewriter.graphemes_left.join("");
    *text = create_dialog_text(current_text, rest);
}
