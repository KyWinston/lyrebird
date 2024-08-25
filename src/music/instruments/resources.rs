use bevy::prelude::*;
use bevy_fundsp::prelude::*;
use uuid::Uuid;

use crate::resources::hh_808_mk;

#[derive(Debug, Resource)]
pub struct HiHatId(pub Uuid);

pub fn hi_hat(env: f32, env_2: f32) -> An<Unit<U0, U2>> {
    unit::<U0, U2>(Box::new(
        hh_808_mk() >> bandrez_hz(env, env_2) >> declick_s(0.001) >> (split::<U2>() * 0.5),
    ))
}
