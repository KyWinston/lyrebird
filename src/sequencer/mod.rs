use bevy::prelude::*;
use resources::SequencePulse;
use systems::play_beat;

use crate::soundfont::resources::SoundFonts;

pub mod components;
pub mod resources;
pub mod systems;

pub struct SequencerPlugin;

impl Plugin for SequencerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PostUpdate,
            (
                (|mut pulse: ResMut<SequencePulse>, time: Res<Time>| pulse.tick_beat(time)),
                play_beat.run_if(resource_exists::<SoundFonts>),
            )
                .run_if(resource_exists::<SequencePulse>),
        );
    }
}
