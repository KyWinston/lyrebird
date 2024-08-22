use bevy::prelude::{Component, Resource};
use bevy_fundsp::prelude::*;
use uuid::Uuid;

#[derive(Resource, Component, Default, Clone)]
pub struct MusicChannel;

pub struct Instrument<F>(pub F);

impl<T: AudioUnit + 'static, F: Send + Sync + 'static + Fn() -> T> DspGraph for Instrument<F> {
    fn id(&self) -> Uuid {
        Uuid::from_u128(0xa1a2a3a4b1b2c1c2d1d2d3d4d5d6d7d8u128)
    }

    fn generate_graph(&self) -> Box<dyn AudioUnit> {
        Box::new((self.0)())
    }
}

#[derive(Resource)]
pub struct EnvelopeConfig(pub Shared, pub Shared);

impl EnvelopeConfig {
    pub fn set_envelope(&self, env: f32, env_2: f32) {
        self.0.set_value(env);
        self.1.set_value(env_2);
    }
}
