use bevy::prelude::*;
use lyrebird::synth::events::PlaySequence;

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((TextBundle {
        text: Text {
            sections: vec![
                TextSection::new(
                    "INSTRUCTIONS \n\
                        0 to 9 - play instrument \n\
                        ",
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 30.0,
                        color: Color::WHITE,
                    },
                ),
                TextSection::from_style(TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 30.0,
                    color: Color::BLACK,
                }),
            ],
            ..default()
        },
        ..default()
    },));
}

pub fn play_instrument(keys: Res<ButtonInput<KeyCode>>, mut tone: EventWriter<PlaySequence>) {
    if keys.just_pressed(KeyCode::Digit0) {
        tone.send(PlaySequence(None));
    } else if keys.just_pressed(KeyCode::Digit1) {
        tone.send(PlaySequence(Some("hi_hat".to_string())));
    } else if keys.just_pressed(KeyCode::Digit2) {
        tone.send(PlaySequence(Some("bass".to_string())));
    } else if keys.just_pressed(KeyCode::Digit3) {
        tone.send(PlaySequence(Some("bass_drum".to_string())));
    } else if keys.just_pressed(KeyCode::Digit4) {
        tone.send(PlaySequence(Some("snare_drum".to_string())));
    }
}
