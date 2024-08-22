use bevy::app::{App, Plugin, Update};
use bevy_fundsp::prelude::*;
use bevy_kira_audio::prelude::*;
use events::{start_track, PlayEvent};
use instruments::resources::{hi_hat, HiHatId};
use resources::{EnvelopeConfig, Instrument, MusicChannel};

pub mod events;
pub mod instruments;
pub mod resources;
mod systems;

// #[derive(Debug, Clone, Copy)]
// enum Pitch {
//     C,
//     Cs,
//     D,
//     Ds,
//     E,
//     F,
//     Fs,
//     G,
//     Gs,
//     A,
//     As,
//     B,
// }

// impl Pitch {
//     fn to_f32(self) -> f32 {
//         match self {
//             Pitch::C => 261.626,
//             Pitch::Cs => 277.183,
//             Pitch::D => 293.665,
//             Pitch::Ds => 311.127,
//             Pitch::E => 329.628,
//             Pitch::F => 349.228,
//             Pitch::Fs => 369.994,
//             Pitch::G => 391.995,
//             Pitch::Gs => 415.305,
//             Pitch::A => 440.0,
//             Pitch::As => 466.164,
//             Pitch::B => 493.883,
//         }
//     }
// }

// impl From<Pitch> for f32 {
//     fn from(pitch: Pitch) -> Self {
//         pitch.to_f32()
//     }
// }
pub struct MusicPlugin;

impl Plugin for MusicPlugin {
    fn build(&self, app: &mut App) {
        let wv_raw: (f32, f32) = (8.0, 20000.0);
        let wv_envelope = (shared(wv_raw.0), shared(wv_raw.1));
        let wv_envelope_2 = wv_envelope.clone();

        let hi_hat = move || hi_hat(wv_raw.0, wv_raw.1);
        let hi_hat_dsp = Instrument(hi_hat);
        let hi_hat_id = hi_hat_dsp.id();
        app.add_event::<PlayEvent>()
            .add_systems(Update, start_track)
            .insert_resource(EnvelopeConfig(wv_envelope_2.0, wv_envelope_2.1))
            .insert_resource::<HiHatId>(HiHatId(hi_hat_id))
            .add_dsp_source(hi_hat_dsp, SourceType::Static { duration: 0.1 })
            .add_audio_channel::<MusicChannel>();
    }
}
