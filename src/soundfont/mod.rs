use bevy::{prelude::*, utils::HashMap};

use bevy_kira_audio::AudioApp;
use resources::{SfChannel, SoundFonts};

pub mod components;
pub mod events;
pub mod resources;
pub mod systems;
pub struct SoundFontPlugin;

impl Plugin for SoundFontPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource::<SoundFonts>(SoundFonts(HashMap::new()))
            .add_audio_channel::<SfChannel>();
    }
}
