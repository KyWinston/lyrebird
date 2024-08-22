use std::time::Instant;

use bevy::prelude::*;
use bevy_yarnspinner::prelude::LocalizedLine;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug, Clone, PartialEq, Resource)]
pub struct Typewriter {
    pub character_name: Option<String>,
    pub current_text: String,
    pub graphemes_left: Vec<String>,
    pub last_before_options: bool,
    elapsed: f32,
    start: Instant,
    fast_typing: bool,
}

impl Default for Typewriter {
    fn default() -> Self {
        Self {
            character_name: default(),
            current_text: default(),
            graphemes_left: default(),
            last_before_options: default(),
            elapsed: default(),
            start: Instant::now(),
            fast_typing: default(),
        }
    }
}

impl Typewriter {
    pub fn set_line(&mut self, line: &LocalizedLine) {
        *self = Self {
            character_name: line.character_name().map(|s| s.to_string()),
            current_text: String::new(),
            graphemes_left: line
                .text_without_character_name()
                .graphemes(true)
                .map(|s| s.to_string())
                .collect(),
            last_before_options: line.is_last_line_before_options(),
            ..default()
        };
    }

    pub fn is_finished(&self) -> bool {
        self.graphemes_left.is_empty() && !self.current_text.is_empty()
    }

    pub fn fast_forward(&mut self) {
        self.fast_typing = true;
    }

    pub fn update_current_text(&mut self) {
        if self.is_finished() {
            return;
        }
        self.elapsed += self.start.elapsed().as_secs_f32();
        self.start = Instant::now();
        let calculated_graphemes = (self.graphemes_per_second() * self.elapsed).floor() as usize;
        let graphemes_left = self.graphemes_left.len();
        let grapheme_length_to_take = (calculated_graphemes).min(graphemes_left);
        self.elapsed -= grapheme_length_to_take as f32 / self.graphemes_per_second();
        let graphemes_to_take = self.graphemes_left.drain(..grapheme_length_to_take);
        self.current_text.extend(graphemes_to_take);
    }

    pub fn graphemes_per_second(&self) -> f32 {
        if self.fast_typing {
            120.0
        } else {
            40.0
        }
    }
}

