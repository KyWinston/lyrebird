use bevy::{
    color::palettes::css::{ALICE_BLUE, TOMATO},
    prelude::*,
};

use crate::dialogue_view::asset::MEDIUM;


pub const DIALOG_WIDTH: f32 = 800.0 * 0.8;
pub const TEXT_BORDER_HORIZONTAL: f32 = 120.0;
pub const TEXT_BORDER_TOP: f32 = 30.0;
pub const TEXT_BORDER_BOTTOM: f32 = TEXT_BORDER_TOP + 10.0;

pub fn standard() -> Style {
    Style {
        max_width: Val::Px(DIALOG_WIDTH - 2.0 * TEXT_BORDER_HORIZONTAL),
        ..default()
    }
}
pub fn options() -> Style {
    const INDENT_MODIFIER: f32 = 1.0;
    Style {
        margin: UiRect::horizontal(Val::Px((INDENT_MODIFIER - 1.0) * TEXT_BORDER_HORIZONTAL)),
        max_width: Val::Px(DIALOG_WIDTH - 2.0 * INDENT_MODIFIER * TEXT_BORDER_HORIZONTAL),
        ..default()
    }
}

pub fn text_standard() -> TextStyle {
    TextStyle {
        font: MEDIUM,
        font_size: 20.0,
        color: Color::WHITE,
    }
}
pub fn text_name() -> TextStyle {
    TextStyle {
        font: MEDIUM,
        font_size: 18.0,
        ..text_standard()
    }
}

pub fn option_id() -> TextStyle {
    TextStyle {
        font: MEDIUM,
        color: ALICE_BLUE.into(),
        ..option_text()
    }
}

pub fn option_text() -> TextStyle {
    TextStyle {
        font_size: 18.0,
        color: TOMATO.into(),
        ..text_standard()
    }
}
