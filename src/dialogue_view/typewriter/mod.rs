use bevy::prelude::*;
use bevy_yarnspinner::{events::*, prelude::*};
use events::TypewriterFinishedEvent;
use resources::Typewriter;
use systems::{bob_continue, despawn, send_finished_event, show_continue, spawn, write_text};

use super::DialogueViewSystemSet;

pub mod events;
pub mod resources;
pub mod systems;

pub const UNNAMED: &str = "unnamed";

pub fn typewriter_plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            send_finished_event.run_if(resource_exists::<Typewriter>),
            despawn.run_if(on_event::<DialogueCompleteEvent>()),
            spawn.run_if(on_event::<DialogueStartEvent>()),
            write_text.run_if(resource_exists::<Typewriter>),
            show_continue.run_if(resource_exists::<Typewriter>),
            bob_continue,
        )
            .chain()
            .after(YarnSpinnerSystemSet)
            .in_set(DialogueViewSystemSet),
    )
    .add_event::<TypewriterFinishedEvent>();
}
