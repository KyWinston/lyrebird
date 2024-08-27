use bevy::prelude::*;
use bevy_kira_audio::{AudioChannel, AudioControl};

use super::resources::{SfChannel, SoundFont};

#[derive(Event)]
pub struct SFplayEvent(pub SoundFont);

pub fn play_sf(
    asset_server: Res<AssetServer>,
    mut sf_play: EventReader<SFplayEvent>,
    audio: Res<AudioChannel<SfChannel>>,
) {
    audio.stop();
    for ev in sf_play.read() {
        let ev = &ev.0;
        audio
            .play(asset_server.load("audio/soundfonts/".to_owned() + &ev.path + ".wav"))
            .with_volume(ev.volume)
            .start_from(ev.target_chunk[0])
            .end_at(ev.target_chunk[1]);
    }
}
