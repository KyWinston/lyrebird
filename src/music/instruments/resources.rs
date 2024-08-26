use bevy::prelude::*;
use fundsp::hacker::*;
use uuid::Uuid;

use crate::resources::hh_808_mk;

#[derive(Debug, Resource)]
pub struct HiHatId(pub Uuid);

pub fn hi_hat() -> An<Unit<U0, U2>> {
    unit::<U0, U2>(Box::new(
        hh_808_mk() >> lowrez_hz(10000.0, 0.7) >> highpass_hz(400.0, 0.1) >> (split::<U2>() * 0.5),
    ))
}
