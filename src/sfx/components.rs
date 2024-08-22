use bevy::prelude::*;

#[derive(Component)]
pub struct SfxEmitter {
    pub sound: String,
    pub intensity: f64,
    pub looped: bool,
}

#[derive(Component)]
pub struct ListeningCamera;
