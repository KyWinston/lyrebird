use bevy::prelude::*;
use lyrebird::soundfont::resources::{SoundFont, SoundFonts};

use crate::resources::SoundfontManifest;

pub fn load_audio(mut soundfonts: ResMut<SoundFonts>, manifest: Res<SoundfontManifest>) {
    for (_, item) in manifest.soundfonts.iter() {
        println!("{:?}", item.name);
        soundfonts.0.insert(
            item.name.clone(),
            SoundFont {
                name: item.name.clone(),
                handle: item.handle.clone(),
                target_chunk: item.target_chunk,
                volume: item.volume,
            },
        );
    }
}
