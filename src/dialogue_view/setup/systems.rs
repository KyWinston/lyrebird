use bevy::{ecs::system::EntityCommands, prelude::*};
use bevy_yarnspinner::prelude::DialogueOption;

use crate::dialogue_view::asset::CONTINUE_INDICATOR;

use super::{
    components::{
        DialogueContinueNode, DialogueNameNode, DialogueNode, OptionButton, OptionsNode, UiRootNode,
    },
    style::{
        self, option_id, option_text, standard, text_name, text_standard, DIALOG_WIDTH,
        TEXT_BORDER_BOTTOM, TEXT_BORDER_HORIZONTAL, TEXT_BORDER_TOP,
    },
};

pub fn setup(mut commands: Commands) {
    commands
        .spawn((
            fmt_name("root"),
            NodeBundle {
                style: Style {
                    display: Display::Grid,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_content: AlignContent::End,
                    justify_content: JustifyContent::SpaceAround,
                    grid_auto_flow: GridAutoFlow::Row,
                    grid_template_columns: vec![RepeatedGridTrack::minmax(
                        1,
                        MinTrackSizingFunction::Auto,
                        MaxTrackSizingFunction::Px(DIALOG_WIDTH),
                    )],

                    grid_auto_rows: vec![GridTrack::min_content()],
                    ..default()
                },
                visibility: Visibility::Hidden,
                ..default()
            },
            UiRootNode,
        ))
        .with_children(|parent| {
            parent.spawn((
                fmt_name("name"),
                TextBundle {
                    text: Text::from_section(String::new(), text_name()),
                    style: Style {
                        margin: UiRect {
                            left: Val::Px(TEXT_BORDER_HORIZONTAL / 2.0),
                            bottom: Val::Px(-8.0),
                            ..default()
                        },
                        ..default()
                    },
                    z_index: ZIndex::Local(1),
                    ..default()
                },
                DialogueNameNode,
                Label,
            ));

            parent
                .spawn((
                    fmt_name("dialogue"),
                    NodeBundle {
                        style: Style {
                            flex_direction: FlexDirection::Column,
                            justify_content: JustifyContent::SpaceAround,
                            align_items: AlignItems::FlexStart,
                            padding: UiRect {
                                top: Val::Px(TEXT_BORDER_TOP),
                                bottom: Val::Px(TEXT_BORDER_BOTTOM),
                                left: Val::Px(TEXT_BORDER_HORIZONTAL),
                                right: Val::Px(TEXT_BORDER_HORIZONTAL),
                            },
                            ..default()
                        },
                        background_color: Color::BLACK.with_alpha(0.8).into(),
                        border_radius: BorderRadius::all(Val::Px(20.0)),
                        ..default()
                    },
                ))
                .with_children(|parent| {
                    parent.spawn((
                        fmt_name("text"),
                        TextBundle::from_section(String::new(), text_standard())
                            .with_style(standard()),
                        DialogueNode,
                        Label,
                    ));
                })
                .with_children(|parent| {
                    // Options
                    parent.spawn((
                        fmt_name("options"),
                        NodeBundle {
                            style: Style {
                                display: Display::None,
                                flex_direction: FlexDirection::Column,
                                justify_content: JustifyContent::FlexEnd,
                                align_items: AlignItems::FlexStart,
                                margin: UiRect::top(Val::Px(20.0)),
                                ..default()
                            },
                            visibility: Visibility::Hidden,
                            ..default()
                        },
                        OptionsNode,
                    ));
                });

            parent.spawn((
                fmt_name("continue indicator"),
                ImageBundle {
                    image: UiImage {
                        texture: CONTINUE_INDICATOR,
                        ..default()
                    },
                    style: Style {
                        justify_self: JustifySelf::Center,
                        align_self: AlignSelf::Center,
                        margin: UiRect {
                            top: Val::Px(-18.),
                            bottom: Val::Px(15.),
                            ..default()
                        },
                        ..default()
                    },
                    z_index: ZIndex::Local(1),
                    visibility: Visibility::Hidden,
                    ..default()
                },
                DialogueContinueNode,
            ));
        });
}

pub fn fmt_name(name: &str) -> Name {
    Name::new(format!("Yarn Spinner example dialogue view node: {name}"))
}

pub fn create_dialog_text(text: impl Into<String>, invisible: impl Into<String>) -> Text {
    Text::from_sections([
        TextSection {
            value: text.into(),
            style: text_standard(),
        },
        TextSection {
            value: invisible.into(),
            style: TextStyle {
                color: Color::NONE,
                ..text_standard()
            },
        },
    ])
}

pub fn spawn_options<'a, T>(entity_commands: &mut EntityCommands, options: T)
where
    T: IntoIterator<Item = &'a DialogueOption>,
    <T as IntoIterator>::IntoIter: 'a,
{
    entity_commands.with_children(|parent| {
        for (i, option) in options.into_iter().enumerate() {
            parent
                .spawn((
                    fmt_name("option button"),
                    ButtonBundle {
                        style: Style {
                            justify_content: JustifyContent::FlexStart,
                            ..default()
                        },
                        image: UiImage::default().with_color(Color::NONE),
                        ..default()
                    },
                    OptionButton(option.id),
                ))
                .with_children(|parent| {
                    let sections = [
                        TextSection {
                            value: format!("{}: ", i + 1),
                            style: option_id(),
                        },
                        TextSection {
                            value: option.line.text.clone(),
                            style: option_text(),
                        },
                    ];

                    parent.spawn((
                        fmt_name("option text"),
                        TextBundle::from_sections(sections).with_style(style::options()),
                        Label,
                    ));
                });
        }
    });
}
