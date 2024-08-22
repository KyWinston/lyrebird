use bevy::prelude::*;
use bevy_yarnspinner::prelude::*;
use events::StartDialogueEvent;
use systems::spawn_dialogue_runner;

pub mod events;
pub mod systems;

pub struct DialoguePlugin;

impl Plugin for DialoguePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<StartDialogueEvent>()
            .add_plugins(YarnSpinnerPlugin::new())
            .add_systems(
                Update,
                spawn_dialogue_runner.run_if(resource_exists::<YarnProject>),
            );
    }
}
