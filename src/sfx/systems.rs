use bevy::{ecs::component::HookContext, prelude::*};
use bevy_kira_audio::{
    AudioChannel, AudioControl, AudioInstance, AudioSource, AudioTween, MainTrack,
};

use super::{
    components::{ListeningCamera, SfxEmitter},
    events::PlayTone,
};

pub fn play_sfx(
    ctx: HookContext,
    mut emitters: Query<&mut SfxEmitter>,
    audio: ResMut<AudioChannel<MainTrack>>,
) {
    if let Ok(mut sfx) = emitters.get_mut(ctx.entity) {
        sfx.instance = audio
            .play(sfx.sound)
            .loop_from(sfx.get_duration().0)
            .handle();
    }
}

pub fn process_sound_properties(
    mut emitters: Query<(&Transform, &mut SfxEmitter)>,
    mut audio: ResMut<Assets<AudioInstance>>,
    listener: Query<(&GlobalTransform, &ListeningCamera)>,
) {
    if let Ok((l_transform, listener)) = listener.single() {
        for (transform, sfx) in emitters.iter_mut() {
            if let Some(sound) = audio.get_mut(sfx.instance.id()) {
                sound.set_playback_rate(sfx.speed, AudioTween::default());
            }
        }
    }
}

pub fn process_spatial_damping(
    mut emitters: Query<(&Transform, &mut SfxEmitter)>,
    mut audio: ResMut<Assets<AudioInstance>>,
    listener: Query<(&GlobalTransform, &ListeningCamera)>,
) {
    if let Ok((l_transform, listener)) = listener.single() {
        for (transform, sfx) in emitters.iter_mut() {
            if let Some(sound) = audio.get_mut(sfx.instance.id()) {
                let sfx_distance = l_transform.translation().distance(transform.translation);
                sound.set_volume(
                    EasingCurve::new(sfx.near, sfx.far, EaseFunction::CubicIn)
                        .sample(sfx_distance)
                        .unwrap_or_else(|| {
                            if sfx_distance < sfx.near {
                                1.0
                            } else if sfx_distance > sfx.far {
                                0.0
                            }
                        }),
                    AudioTween::default(),
                );
            }
        }
    }
}
