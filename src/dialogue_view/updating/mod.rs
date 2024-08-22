use bevy::prelude::*;
use bevy_yarnspinner::{events::*, prelude::*};
use events::SpeakerChangeEvent;
use systems::{continue_dialogue, hide_dialog, present_line, present_options, show_dialog};

use super::{typewriter::{resources::Typewriter, systems::spawn}, DialogueViewSystemSet};

pub mod events;
pub mod systems;
pub fn ui_updating_plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            hide_dialog,
            show_dialog.run_if(on_event::<DialogueStartEvent>()),
            present_line
                .run_if(resource_exists::<Typewriter>.and_then(on_event::<PresentLineEvent>())),
            present_options.run_if(on_event::<PresentOptionsEvent>()),
            continue_dialogue.run_if(resource_exists::<Typewriter>),
        )
            .chain()
            .after(YarnSpinnerSystemSet)
            .after(spawn)
            .in_set(DialogueViewSystemSet),
    )
    .add_event::<SpeakerChangeEvent>()
    .register_type::<SpeakerChangeEvent>();
}
