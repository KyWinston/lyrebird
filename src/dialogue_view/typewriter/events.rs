use bevy::prelude::*;

#[derive(Debug, Eq, PartialEq, Hash, Reflect, Event)]
pub struct TypewriterFinishedEvent;
