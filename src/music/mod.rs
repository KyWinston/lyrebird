// use bevy::app::{App, Plugin, Update};
// use events::{InstrumentPlayEvent, TrackPlayEvent};
// use resources::MusicChannel;
use bevy::prelude::*;

pub mod events;
pub mod instruments;
pub mod resources;
mod systems;

pub struct MusicPlugin;

impl Plugin for MusicPlugin {
    fn build(&self, _app: &mut App) {
        // app.add_event::<TrackPlayEvent>()
        //     .add_event::<InstrumentPlayEvent>()
            // .add_systems(Update, start_track)
            // .add_audio_channel::<MusicChannel>();
    }
}
