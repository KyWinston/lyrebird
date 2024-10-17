use bevy::prelude::*;

use crate::sfx::components::ListeningCamera;

pub fn setup_cam_audio(_trigger: Trigger<OnAdd, ListeningCamera>, _commands: Commands) {
    // commands.entity(trigger.entity()).insert(AudioReceiver);
}
