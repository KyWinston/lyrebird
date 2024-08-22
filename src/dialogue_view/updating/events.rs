use bevy::prelude::*;

#[derive(Debug, Eq, PartialEq, Hash, Reflect, Event)]
#[reflect(Debug, PartialEq, Hash)]
pub struct SpeakerChangeEvent {
    /// The name of the character who is or was speaking.
    pub character_name: String,
    /// If `true`, the character just started speaking. Otherwise, they just stopped.
    pub speaking: bool,
}
