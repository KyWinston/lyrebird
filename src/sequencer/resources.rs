use std::time::Duration;

use bevy::prelude::*;
// utils::hashbrown::HashMap};

// use crate::soundfont::resources::SoundFont;

#[derive(Resource)]
pub struct SequencePulse {
    bpm: u32,
    beats_per_bar: u8,
    pulse: Timer,
    current_beat: u8,
    elapsed_beats: u32,
    // instruments: HashMap<String, SoundFont>,
}

impl SequencePulse {
    pub fn from_bpm(bpm: u32, beats_per_bar: u8) -> Self {
        Self {
            bpm,
            beats_per_bar,
            current_beat: 0,
            pulse: Timer::new(
                Duration::from_secs_f32(60.0 / bpm as f32),
                TimerMode::Repeating,
            ),
            elapsed_beats: 0,
            // instruments: HashMap::new(),
        }
    }

    pub fn tick_beat(&mut self, time: Res<Time>) {
        self.pulse.tick(time.delta());
        if self.pulse.finished() {
            self.current_beat += 1;
            self.elapsed_beats += 1;
            if self.current_beat >= self.beats_per_bar {
                self.current_beat = 0;
            }
        }
    }
    pub fn check_beat(&self) -> bool {
        self.pulse.finished()
    }
    pub fn tempo(&self) -> u128 {
        ((60.0 / self.bpm as f32) * 1000.0) as u128
    }
}
