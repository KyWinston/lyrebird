use bevy::prelude::*;
use bevy_kira_audio::{AudioChannel, AudioControl};

use super::resources::{SfChannel, SoundFont};

pub fn play_sf(sf: &SoundFont, audio: Res<AudioChannel<SfChannel>>) {
    audio.stop();
    audio
        .play(sf.handle.clone())
        .with_volume(sf.volume)
        .start_from(sf.target_chunk[0])
        .end_at(sf.target_chunk[1]);
}
