use bevy::prelude::*;
use lyrebird::synth::events::PlayTone;

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
            ..Default::default()
        },
        ..default()
    },));
}

pub fn play_instrument(keys: Res<ButtonInput<KeyCode>>, mut tone: EventWriter<PlayTone>) {
    if keys.just_pressed(KeyCode::Digit0) {
        tone.send(PlayTone([145, 60, 127], "hi_hat".to_string(), None));
    } else if keys.just_pressed(KeyCode::Digit1) {
        tone.send(PlayTone([145, 60, 127], "hi_hat".to_string(), Some(0)));
    } else if keys.just_pressed(KeyCode::Digit2) {
        tone.send(PlayTone([145, 60, 127], "hi_hat".to_string(), Some(1)));
    } else if keys.just_pressed(KeyCode::Digit3) {
        tone.send(PlayTone([145, 60, 127], "hi_hat".to_string(), Some(2)));
    }
}
