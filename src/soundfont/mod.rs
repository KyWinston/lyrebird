use bevy::prelude::*;

use bevy_kira_audio::AudioApp;
use events::{play_sf, SFplayEvent};
use resources::SfChannel;

pub mod components;
pub mod events;
pub mod resources;
pub mod systems;
pub struct SoundFontPlugin;

impl Plugin for SoundFontPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SFplayEvent>()
            .add_systems(Update, play_sf)
            .add_audio_channel::<SfChannel>();
    }
}
