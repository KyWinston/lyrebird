use bevy::{prelude::*, utils::HashMap};
use leafwing_manifest::{
    identifier::Id,
    manifest::{Manifest, ManifestFormat},
};
use lyrebird::soundfont::resources::SoundFont;
use serde::{Deserialize, Serialize};

#[derive(Debug, Resource, PartialEq)]
pub struct SoundfontManifest {
    pub soundfonts: HashMap<Id<SoundFont>, SoundFont>,
}

impl Manifest for SoundfontManifest {
    type Item = SoundFont;
    type RawItem = RawSoundFont;
    type RawManifest = RawSoundfontManifest;
    type ConversionError = std::convert::Infallible;

    const FORMAT: ManifestFormat = ManifestFormat::Ron;

    fn get(&self, id: Id<SoundFont>) -> Option<&Self::Item> {
        self.soundfonts.get(&id)
    }

    fn from_raw_manifest(
        raw_manifest: Self::RawManifest,
        world: &mut World,
    ) -> Result<Self, Self::ConversionError> {
        let soundfonts: HashMap<_, _> = raw_manifest
            .soundfonts
            .into_iter()
            .map(|raw_item| {
                let handle = world.load_asset("soundfonts/".to_string() + &raw_item.path + ".wav");

                let item = SoundFont {
                    name: raw_item.name.clone(),
                    handle,
                    target_chunk: [raw_item.start_chunk, raw_item.end_chunk],
                    volume: raw_item.volume,
                };

                (Id::from_name(&raw_item.name), item)
            })
            .collect();

        Ok(SoundfontManifest { soundfonts })
    }
}

#[derive(Debug, Asset, TypePath, Serialize, Deserialize, PartialEq)]
pub struct RawSoundfontManifest {
    soundfonts: Vec<RawSoundFont>,
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct RawSoundFont {
    pub name: String,
    pub path: String,
    pub start_chunk: f64,
    pub end_chunk: f64,
    pub volume: f64,
}
