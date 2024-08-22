use bevy::asset::load_internal_binary_asset;
use bevy::prelude::*;
use systems::{load_font, load_image};

pub const MEDIUM: Handle<Font> = Handle::weak_from_u128(2263821398159872327);
pub const CONTINUE_INDICATOR: Handle<Image> = Handle::weak_from_u128(5464879846123416874);

pub mod systems;

pub fn ui_assets_plugin(app: &mut App) {
    load_internal_binary_asset!(
        app,
        MEDIUM,
        "../../../assets/fonts/FiraSans-Bold.ttf",
        load_font
    );

    load_internal_binary_asset!(
        app,
        CONTINUE_INDICATOR,
        "../../../assets/arrow_basic_s.png",
        load_image
    );
}
