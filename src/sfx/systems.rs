use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

use super::components::SfxEmitter;

pub fn play_sfx(
    trigger: Trigger<OnAdd, SfxEmitter>,
    mut commands: Commands,
    query: Query<(Entity, &SfxEmitter)>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
) {
    let (_ent, emitter) = query.get(trigger.entity()).unwrap();
    let mut sfx = audio.play(asset_server.load("audio/sfx/".to_string() + &emitter.sound));
    sfx.with_volume(emitter.intensity)
        .start_from(emitter.get_duration().0)
        .end_at(emitter.get_duration().1);

    if emitter.looped {
        sfx.looped();
    }

    commands.entity(trigger.entity()).insert(AudioEmitter {
        instances: vec![sfx.handle()],
    });
}
