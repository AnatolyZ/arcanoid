use super::components::ButtonType;
use crate::main_menu::components::MainMenu;
use crate::states::GameState;
use crate::textures::resources::Textures;
use bevy::prelude::*;

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const PRESSED_BUTTON: Color = Color::srgb(0.1, 0.1, 0.1);
const HOVERED_BUTTON: Color = Color::srgb(0.2, 0.2, 0.2);

pub fn spawn_main_menu(commands: Commands, textures: Res<Textures>) {
    build_main_menu(commands, textures);
}

pub fn despawn_main_menu(mut commands: Commands, main_menu_query: Query<Entity, With<MainMenu>>) {
    for entity in main_menu_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn build_main_menu(mut commands: Commands, textures: Res<Textures>) {
    // Main menu container
    let main_menu_node = NodeBundle {
        background_color: Srgba::hex("#7f8a88").unwrap().into(),
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::SpaceBetween,
            align_items: AlignItems::Center,
            ..Default::default()
        },
        ..Default::default()
    };

    // Caption
    let text_caption_node = TextBundle {
        text: Text {
            sections: vec![TextSection {
                value: "ARCANOID".to_string(),
                style: TextStyle {
                    font: textures.font.clone(),
                    font_size: 160.0,
                    color: Color::WHITE,
                },
            }],
            justify: JustifyText::Center,
            linebreak_behavior: bevy::text::BreakLineOn::NoWrap,
        },
        style: Style {
            margin: UiRect::all(Val::Px(10.0)),
            ..Default::default()
        },
        ..Default::default()
    };

    // Start button
    let button_start_box_node = NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(20.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            margin: UiRect::all(Val::Px(10.0)),
            ..default()
        },
        ..default()
    };
    let button_start_node = ButtonBundle {
        style: Style {
            width: Val::Percent(40.0),
            height: Val::Percent(90.0),
            border: UiRect::all(Val::Px(5.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,

            ..default()
        },
        border_color: BorderColor(Color::BLACK),
        background_color: NORMAL_BUTTON.into(),
        ..default()
    };
    let button_start_text_node = TextBundle::from_section(
        "Start",
        TextStyle {
            font_size: 80.0,
            font: textures.font.clone(),
            color: Color::srgb(0.9, 0.9, 0.9),
        },
    );

    // Options button
    let button_options_box_node = NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(20.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            margin: UiRect::all(Val::Px(10.0)),
            ..default()
        },
        ..default()
    };
    let button_options_node = ButtonBundle {
        style: Style {
            width: Val::Percent(40.0),
            height: Val::Percent(90.0),
            border: UiRect::all(Val::Px(5.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        border_color: BorderColor(Color::BLACK),
        background_color: NORMAL_BUTTON.into(),
        ..default()
    };
    let button_options_text_node = TextBundle::from_section(
        "Options",
        TextStyle {
            font_size: 80.0,
            font: textures.font.clone(),
            color: Color::srgb(0.9, 0.9, 0.9),
        },
    );

    // Exit button
    let button_exit_box_node = NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(20.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            margin: UiRect::all(Val::Px(10.0)),
            ..default()
        },
        ..default()
    };
    let button_exit_node = ButtonBundle {
        style: Style {
            width: Val::Percent(40.0),
            height: Val::Percent(90.0),
            border: UiRect::all(Val::Px(5.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        border_color: BorderColor(Color::BLACK),
        background_color: NORMAL_BUTTON.into(),
        ..default()
    };
    let button_exit_text_node = TextBundle::from_section(
        "Exit",
        TextStyle {
            font_size: 80.0,
            font: textures.font.clone(),
            color: Color::srgb(0.9, 0.9, 0.9),
        },
    );

    let footer_node = NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(20.0),
            justify_content: JustifyContent::SpaceBetween,
            align_items: AlignItems::Center,
            margin: UiRect::all(Val::Px(10.0)),
            flex_direction: FlexDirection::Row,
            ..default()
        },
        ..default()
    };

    let bevy_logo_node = ImageBundle {
        style: Style {
            width: Val::Px(90.0),
            height: Val::Px(66.0),
            margin: UiRect::all(Val::Px(10.0)),
            ..default()
        },
        image: UiImage {
            texture: textures.bevy_logo.clone(),
            ..default()
        },
        ..default()
    };

    let rapier_logo_node = ImageBundle {
        style: Style {
            width: Val::Px(263.0),
            height: Val::Px(66.0),
            margin: UiRect::all(Val::Px(10.0)),
            ..default()
        },
        image: UiImage {
            texture: textures.rapier_logo.clone(),
            ..default()
        },
        ..default()
    };

    let text_caption = commands.spawn(text_caption_node).id();
    let button_start = commands
        .spawn(button_start_box_node)
        .with_children(|parent| {
            parent
                .spawn((button_start_node, ButtonType::Start))
                .clear_children()
                .with_children(|parent| {
                    parent.spawn(button_start_text_node);
                });
        })
        .id();
    let button_options = commands
        .spawn(button_options_box_node)
        .with_children(|parent| {
            parent.spawn(button_options_node).with_children(|parent| {
                parent.spawn(button_options_text_node);
            });
        })
        .id();
    let button_exit = commands
        .spawn(button_exit_box_node)
        .with_children(|parent| {
            parent
                .spawn((button_exit_node, ButtonType::Exit))
                .with_children(|parent| {
                    parent.spawn(button_exit_text_node);
                });
        })
        .id();

    let footer = commands
        .spawn(footer_node)
        .with_children(|parent| {
            parent.spawn(bevy_logo_node);
            parent.spawn(rapier_logo_node);
        })
        .id();

    let main_menu = commands.spawn((main_menu_node, MainMenu {})).id();
    commands.entity(main_menu).push_children(&[
        text_caption,
        button_start,
        button_options,
        button_exit,
        footer,
    ]);
}

#[allow(clippy::type_complexity)]
pub fn buttons_state_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &mut Style,
        ),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color, mut border_color, mut style) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                border_color.0 = Color::srgb(1.0, 0.0, 0.0);
                style.height = Val::Percent(89.0);
                style.width = Val::Percent(39.0);
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
                border_color.0 = Color::WHITE;
                style.height = Val::Percent(91.0);
                style.width = Val::Percent(41.0);
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
                border_color.0 = Color::BLACK;
                style.height = Val::Percent(90.0);
                style.width = Val::Percent(40.0);
            }
        }
    }
}

#[allow(clippy::type_complexity)]
pub fn buttons_press_system(
    mut next_state: ResMut<NextState<GameState>>,
    mut interaction_query: Query<(&Interaction, &ButtonType), (Changed<Interaction>, With<Button>)>,
) {
    for (interaction, button_type) in &mut interaction_query {
        if interaction == &Interaction::Pressed {
            match button_type {
                ButtonType::Start => next_state.set(GameState::Setup),
                ButtonType::Exit => {
                    std::process::exit(0);
                }
            }
        }
    }
}
