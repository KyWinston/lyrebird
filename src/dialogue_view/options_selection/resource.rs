use bevy::prelude::*;
use bevy_yarnspinner::prelude::DialogueOption;

#[derive(Debug, Clone, PartialEq, Default, Resource)]
pub struct OptionSelection {
   pub options: Vec<DialogueOption>,
}