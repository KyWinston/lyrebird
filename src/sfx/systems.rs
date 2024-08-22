use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

use super::components::SfxEmitter;

pub fn play_sfx(
    trigger: Trigger<OnAdd, SfxEmitter>,
    mut commands: Commands,
    query: Query<&SfxEmitter>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
) {
    let emitter = query.get(trigger.entity()).unwrap();
    let mut sfx = audio.play(asset_server.load("audio/sfx/".to_string() + &emitter.sound));
    if emitter.looped {
        sfx.looped();
    }

    commands.entity(trigger.entity()).insert(AudioEmitter {
        instances: vec![sfx.handle()],
    });
}
