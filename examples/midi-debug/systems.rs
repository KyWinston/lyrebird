use bevy::prelude::*;
use lyrebird::midi_keys::components::Instructions;

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d::default());
    commands.spawn((
        Text::new(
            "INSTRUCTIONS \n\
            R - Refresh ports \n\
            0 to 9 - Connect to port \n\
            Escape - Disconnect from current port \n",
        ),
        TextFont {
            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
            font_size: 30.0,
            ..default()
        },
    ));
    commands.spawn((
        Instructions,
        Text::new(""),
        TextFont {
            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
            font_size: 30.0,
            ..default()
        },
    ));
}
