use bevy::prelude::*;
use bevy_kira_audio::AudioChannel;

use crate::soundfont::{
    events::play_sf,
    resources::{SfChannel, SoundFonts},
};

pub const UNNAMED: &str = "unnamed";


use super::resources::SequencePulse;

pub fn play_beat(
    pulse: Res<SequencePulse>,
    soundfonts: Res<SoundFonts>,
    audio: Res<AudioChannel<SfChannel>>,
) {
    let sf = &soundfonts.0;
    if sf.contains_key(UNNAMED) && pulse.check_beat() {
        let mut sf = sf[UNNAMED].clone();
        sf.volume = 1.0;
        play_sf(&soundfonts.0[UNNAMED], audio);
    }
}
