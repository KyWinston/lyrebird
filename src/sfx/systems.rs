use bevy::{
    ecs::{component::HookContext, world::DeferredWorld},
    prelude::*,
};
use bevy_kira_audio::{AudioChannel, AudioControl, AudioInstance, AudioTween, MainTrack};

use super::components::{ListeningCamera, SfxEmitter};

pub fn play_sfx(world: DeferredWorld, HookContext { entity, .. }: HookContext) {
    let audio = world.resource::<AudioChannel<MainTrack>>();
    let sfx = world.get::<SfxEmitter>(entity).unwrap();
    audio
        .play(sfx.sound.clone())
        .loop_from(sfx.get_duration().0)
        .handle();
}
pub fn process_sound_properties(
    mut emitters: Query<(&Transform, &mut SfxEmitter)>,
    mut audio: ResMut<Assets<AudioInstance>>,
    listener: Query<(&GlobalTransform, &ListeningCamera)>,
) {
    if let Ok((l_transform, listener)) = listener.single() {
        for (transform, sfx) in emitters.iter_mut() {
            if let Some(inst) = &sfx.instance {
                if let Some(sound) = audio.get_mut(inst.id()) {
                    sound.set_playback_rate(sfx.speed.into(), AudioTween::default());
                }
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
            if let Some(inst) = &sfx.instance {
                if let Some(sound) = audio.get_mut(inst.id()) {
                    let sfx_distance = l_transform.translation().distance(transform.translation);
                    sound.set_volume(
                        EasingCurve::new(sfx.near, sfx.far, EaseFunction::CubicIn)
                            .sample(sfx_distance)
                            .unwrap_or_else(|| {
                                if sfx_distance < sfx.near {
                                    1.0
                                } else if sfx_distance > sfx.far {
                                    0.0
                                } else {
                                    0.0
                                }
                            }) as f64,
                        AudioTween::default(),
                    );
                }
            }
        }
    }
}
