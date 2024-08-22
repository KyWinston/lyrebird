use bevy::prelude::*;
use bevy_yarnspinner::prelude::YarnProject;

use super::events::StartDialogueEvent;

pub fn spawn_dialogue_runner(
    mut talk_ev: EventReader<StartDialogueEvent>,
    mut commands: Commands,
    project: Res<YarnProject>,
) {
    for _ev in talk_ev.read() {
        let mut dialogue_runner = project.create_dialogue_runner();
        dialogue_runner.start_node("Hello");
        commands.spawn(dialogue_runner);
    }
}
