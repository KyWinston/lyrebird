use bevy::prelude::*;

pub const TUNING: f32 = 440.0;

#[derive(Component)]
pub struct Instructions;

pub const KEY_PORT_MAP: [(KeyCode, usize); 10] = [
    (KeyCode::Digit0, 0),
    (KeyCode::Digit1, 1),
    (KeyCode::Digit2, 2),
    (KeyCode::Digit3, 3),
    (KeyCode::Digit4, 4),
    (KeyCode::Digit5, 5),
    (KeyCode::Digit6, 6),
    (KeyCode::Digit7, 7),
    (KeyCode::Digit8, 8),
    (KeyCode::Digit9, 9),
];

#[derive(Debug, Clone, Copy)]
pub enum Pitch {
    C4,
    Cs4,
    D4,
    Ds4,
    E4,
    F4,
    Fs4,
    G4,
    Gs4,
    A4,
    As4,
    B4,
    C5,
    Cs5,
    D5,
    Ds5,
    E5,
    F5,
    Fs5,
    G5,
    Gs5,
    A5,
    As5,
    B5,
}

impl Pitch {
    pub fn to_hz(midi: usize) -> f32 {
        TUNING * (f32::powf(2.0, (midi as f32 - 69.0) / 12.0))
    }
    pub fn frequency(&self) -> f32 {
        match self {
            Pitch::B5 => 987.77,
            Pitch::As5 => 932.33,
            Pitch::A5 => 880.0,
            Pitch::Gs5 => 830.61,
            Pitch::G5 => 783.99,
            Pitch::Fs5 => 739.99,
            Pitch::F5 => 698.46,
            Pitch::E5 => 659.26,
            Pitch::Ds5 => 622.25,
            Pitch::D5 => 587.33,
            Pitch::Cs5 => 554.37,
            Pitch::C5 => 523.25,
            Pitch::B4 => 493.88,
            Pitch::As4 => 466.16,
            Pitch::A4 => 440.0,
            Pitch::Gs4 => 523.25,
            Pitch::G4 => 554.37,
            Pitch::Fs4 => 587.33,
            Pitch::F4 => 349.23,
            Pitch::E4 => 329.63,
            Pitch::Ds4 => 311.13,
            Pitch::D4 => 293.66,
            Pitch::Cs4 => 277.18,
            Pitch::C4 => 261.63,
        }
    }
}
