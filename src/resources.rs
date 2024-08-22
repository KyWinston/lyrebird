use bevy::prelude::*;
use bevy_fundsp::prelude::{
    hacker::{square_hz, unit},
    prelude::U1,
    An, Unit, U0,
};
use std::marker::PhantomData;

#[derive(Resource)]
pub struct ChannelAudioState<T> {
    pub stopped: bool,
    pub paused: bool,
    pub loop_started: bool,
    pub volume: f64,
    _marker: PhantomData<T>,
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

///common generators
pub fn hh_808() -> An<Unit<U0, U1>> {
    unit::<U0, U1>(Box::new(
        square_hz(245.0)
            + square_hz(306.0)
            + square_hz(365.0)
            + square_hz(415.0)
            + square_hz(437.0)
            + square_hz(619.0),
    ))
}

pub fn hh_808_mk() -> An<Unit<U0, U1>> {
    //Moritz Klein hi-hat variation: https://www.youtube.com/watch?v=zbBY7JL9nnQ&t=1424s
    unit::<U0, U1>(Box::new(
        square_hz(120.0)
            + square_hz(150.0)
            + square_hz(180.0)
            + square_hz(219.0)
            + square_hz(240.0)
            + square_hz(261.0),
    ))
}
