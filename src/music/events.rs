use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

use super::resources::MusicChannel;

#[derive(Event)]
pub struct TrackPlayEvent(pub String);

#[derive(Event)]
pub struct InstrumentPlayEvent(pub (f32, f32), pub String);

pub fn start_track(
    mut play_ev: EventReader<TrackPlayEvent>,
    asset_server: Res<AssetServer>,
    audio: Res<AudioChannel<MusicChannel>>,
) {
    for ev in play_ev.read() {
        audio.play(asset_server.load("audio/".to_string() + &ev.0));
    }
}
