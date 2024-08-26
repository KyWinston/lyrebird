use bevy::prelude::*;
use bevy_kira_audio::{AudioChannel, AudioControl};

use super::resources::SfChannel;

#[derive(Event)]
pub struct SFplayEvent(String);

pub fn play_sf(
    asset_server: Res<AssetServer>,
    mut sf_play: EventReader<SFplayEvent>,
    audio: Res<AudioChannel<SfChannel>>,
) {
    for ev in sf_play.read() {
        audio.play(asset_server.load(ev.0.clone())).with_volume(0.3);
    }
}
