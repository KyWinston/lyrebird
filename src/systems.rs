use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

use crate::sfx::components::ListeningCamera;

pub fn setup_cam_audio(trigger: Trigger<OnAdd, ListeningCamera>, mut commands: Commands) {
    commands.entity(trigger.entity()).insert(AudioReceiver);
}
