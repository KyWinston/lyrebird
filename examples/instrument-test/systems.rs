use bevy::prelude::*;
use lyrebird::synth::events::PlaySequence;

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d::default());
    commands.spawn((
        Text::new(
            "INSTRUCTIONS \n\
                        0 to 9 - play instrument \n\
                        ",
        ),
        TextFont {
            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
            font_size: 30.0,
            ..default()
        },
    ));
}

pub fn play_instrument(keys: Res<ButtonInput<KeyCode>>, mut tone: EventWriter<PlaySequence>) {
    if keys.just_pressed(KeyCode::Digit0) {
        tone.send(PlaySequence(None));
    } else if keys.just_pressed(KeyCode::Digit1) {
        tone.send(PlaySequence(Some("hi_hat".to_string())));
    } else if keys.just_pressed(KeyCode::Digit2) {
        tone.send(PlaySequence(Some("piano".to_string())));
    } else if keys.just_pressed(KeyCode::Digit3) {
        tone.send(PlaySequence(Some("bass".to_string())));
    } else if keys.just_pressed(KeyCode::Digit4) {
        tone.send(PlaySequence(Some("bass_drum".to_string())));
    } else if keys.just_pressed(KeyCode::Digit5) {
        tone.send(PlaySequence(Some("snare_drum".to_string())));
    }
}
