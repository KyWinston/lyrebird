use bevy::{
    color::palettes::css::TOMATO, prelude::*, utils::hashbrown::HashMap, window::PrimaryWindow,
};
use bevy_yarnspinner::{events::DialogueCompleteEvent, prelude::DialogueRunner};

use crate::dialogue_view::{
    setup::{
        components::{DialogueNode, OptionButton, OptionsNode, UiRootNode},
        systems::spawn_options,
    },
    typewriter::{events::TypewriterFinishedEvent, resources::Typewriter},
};

use super::{events::HasSelectedOptionEvent, resource::OptionSelection, NUMBER_KEYS, NUMPAD_KEYS};

pub fn create_options(
    option_selection: Res<OptionSelection>,
    mut commands: Commands,
    children: Query<&Children>,
    mut options_node: Query<(Entity, &mut Style, &mut Visibility), With<OptionsNode>>,
    mut root_visibility: Query<&mut Visibility, (With<UiRootNode>, Without<OptionsNode>)>,
) {
    let (entity, mut style, mut visibility) = options_node.single_mut();
    style.display = Display::Flex;
    *visibility = Visibility::Hidden;
    if children.iter_descendants(entity).next().is_none() {
        *root_visibility.single_mut() = Visibility::Inherited;
        let mut entity_commands = commands.entity(entity);
        spawn_options(&mut entity_commands, &option_selection.options);
    }
}

pub fn show_options(
    mut typewriter_finished_event: EventReader<TypewriterFinishedEvent>,
    mut options_node: Query<&mut Visibility, With<OptionsNode>>,
) {
    for _event in typewriter_finished_event.read() {
        let mut visibility = options_node.single_mut();
        *visibility = Visibility::Inherited;
    }
}
#[allow(clippy::type_complexity)]
#[allow(clippy::too_many_arguments)]
pub fn select_option(
    keys: Res<ButtonInput<KeyCode>>,
    typewriter: Res<Typewriter>,
    mut buttons: Query<
        (&Interaction, &OptionButton, &Children),
        (With<Button>, Changed<Interaction>),
    >,
    mut dialogue_runners: Query<&mut DialogueRunner>,
    mut text: Query<&mut Text, Without<DialogueNode>>,
    option_selection: Res<OptionSelection>,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
    mut selected_option_event: EventWriter<HasSelectedOptionEvent>,
) {
    if !typewriter.is_finished() {
        return;
    }

    let mut selection = None;
    let key_to_option: HashMap<_, _> = NUMBER_KEYS
        .into_iter()
        .zip(NUMPAD_KEYS)
        .zip(option_selection.options.iter().map(|option| option.id))
        .collect();
    for ((num_key, numpad_key), option) in key_to_option {
        if keys.just_pressed(num_key) || keys.just_pressed(numpad_key) {
            selection = Some(option);
            break;
        }
    }
    let mut window = windows.single_mut();
    for (interaction, button, children) in buttons.iter_mut() {
        let (color, icon) = match *interaction {
            Interaction::Pressed if selection.is_none() => {
                selection = Some(button.0);
                (TOMATO.into(), CursorIcon::Default)
            }
            Interaction::Hovered => (Color::WHITE, CursorIcon::Pointer),
            _ => (TOMATO.into(), CursorIcon::Default),
        };
        window.cursor.icon = icon;
        let text_entity = children.iter().find(|&e| text.contains(*e)).unwrap();
        let mut text = text.get_mut(*text_entity).unwrap();
        text.sections[1].style.color = color;
    }
    let has_selected_id = selection.is_some();
    if let Some(id) = selection {
        for mut dialogue_runner in dialogue_runners.iter_mut() {
            dialogue_runner.select_option(id).unwrap();
        }
    }
    if has_selected_id {
        selected_option_event.send(HasSelectedOptionEvent);
    }
}

pub fn despawn_options(
    mut has_selected_option_event: EventReader<HasSelectedOptionEvent>,
    mut dialogue_complete_event: EventReader<DialogueCompleteEvent>,
    mut commands: Commands,
    mut options_node: Query<(Entity, &mut Style, &mut Visibility), With<OptionsNode>>,
    mut dialogue_node_text: Query<&mut Text, With<DialogueNode>>,
    mut root_visibility: Query<&mut Visibility, (With<UiRootNode>, Without<OptionsNode>)>,
) {
    let should_despawn =
        !has_selected_option_event.is_empty() || !dialogue_complete_event.is_empty();
    if !should_despawn {
        return;
    }
    has_selected_option_event.clear();
    dialogue_complete_event.clear();
    commands.remove_resource::<OptionSelection>();
    let (entity, mut style, mut visibility) = options_node.single_mut();
    commands.entity(entity).despawn_descendants();
    style.display = Display::None;
    *visibility = Visibility::Hidden;
    *dialogue_node_text.single_mut() = Text::default();
    *root_visibility.single_mut() = Visibility::Hidden;
}
