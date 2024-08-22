use widgets::ui_assets_plugin;
use bevy::prelude::*;
use bevy_yarnspinner::prelude::YarnSpinnerPlugin;
use options_selection::option_selection_plugin;
use setup::ui_setup_plugin;
use typewriter::typewriter_plugin;
use updating::ui_updating_plugin;

#[derive(Debug, Default)]
pub struct DialogueViewPlugin;

#[derive(Debug, Default, Clone, Copy, SystemSet, Eq, PartialEq, Hash)]
pub struct DialogueViewSystemSet;

impl DialogueViewPlugin {
    pub fn new() -> Self {
        Self
    }
}

pub mod widgets;
pub mod options_selection;
pub mod setup;
pub mod typewriter;
pub mod updating;

impl Plugin for DialogueViewPlugin {
    fn build(&self, app: &mut App) {
        assert!(app.is_plugin_added::<YarnSpinnerPlugin>());
        app.add_plugins((
            ui_assets_plugin,
            ui_setup_plugin,
            ui_updating_plugin,
            typewriter_plugin,
            option_selection_plugin,
        ));
    }
}
