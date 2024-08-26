use bevy::app::{App, Plugin, Update};
use bevy_kira_audio::prelude::*;
use events::{start_track, InstrumentPlayEvent, TrackPlayEvent};
use resources::MusicChannel;

pub mod events;
pub mod instruments;
pub mod resources;
mod systems;

pub struct MusicPlugin;

impl Plugin for MusicPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<TrackPlayEvent>()
            .add_event::<InstrumentPlayEvent>()
            .add_systems(Update, start_track)
            .add_audio_channel::<MusicChannel>();
    }
}
