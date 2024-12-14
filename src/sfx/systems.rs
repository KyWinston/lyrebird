use bevy::prelude::*;
use bevy_kira_audio::{AudioChannel, AudioControl, AudioSource, MainTrack};

use super::events::PlayTone;

pub fn play_sfx(
    mut tone: EventReader<PlayTone>,
    asset_server: Res<AssetServer>,
    audio: Res<AudioChannel<MainTrack>>,
) {
    for tone in tone.read() {
        let sfx: Handle<AudioSource> = asset_server.load(format!("audio/sfx/{}", &tone.0));
        audio.play(sfx);
    }
}
