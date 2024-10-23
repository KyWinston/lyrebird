use bevy::prelude::*;
use std::marker::PhantomData;

#[derive(Resource)]
pub struct ChannelAudioState<T> {
    pub stopped: bool,
    pub paused: bool,
    pub loop_started: bool,
    pub volume: f64,
    pub _marker: PhantomData<T>,
}

impl<T> Default for ChannelAudioState<T> {
    fn default() -> Self {
        ChannelAudioState {
            volume: 1.0,
            stopped: true,
            loop_started: false,
            paused: false,
            _marker: PhantomData::<T>,
        }
    }
}
