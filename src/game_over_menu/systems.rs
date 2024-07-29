use super::components::ButtonType;
use crate::game_over_menu::components::GameOverMenu;
use crate::states::GameState;
use crate::textures::resources::Textures;
use bevy::prelude::*;

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const PRESSED_BUTTON: Color = Color::srgb(0.1, 0.1, 0.1);
const HOVERED_BUTTON: Color = Color::srgb(0.2, 0.2, 0.2);

pub fn spawn_game_over_menu(commands: Commands, textures: Res<Textures>) {
    build_game_over_menu(commands, textures);
}

pub fn despawn_game_over_menu(
    mut commands: Commands,
    game_over_menu_query: Query<Entity, With<GameOverMenu>>,
) {
    for entity in game_over_menu_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn build_game_over_menu(mut commands: Commands, textures: Res<Textures>) {
    // Main menu container
    let main_menu_node = NodeBundle {
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
                value: "Game Over".to_string(),
                style: TextStyle {
                    font: textures.font.clone(),
                    font_size: 140.0,
                    color: Color::BLACK,
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

    // Padding 10%
    let padding_node = NodeBundle {
        style: Style {
            height: Val::Percent(10.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        ..default()
    };
    // Restart button
    let button_restart_box_node = NodeBundle {
        style: Style {
            width: Val::Percent(80.0),
            height: Val::Percent(15.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            margin: UiRect::all(Val::Px(10.0)),
            ..default()
        },
        ..default()
    };
    let button_restart_node = ButtonBundle {
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
    let button_restart_text_node = TextBundle::from_section(
        "Restart",
        TextStyle {
            font_size: 56.0,
            font: textures.font.clone(),
            color: Color::srgb(0.9, 0.9, 0.9),
        },
    );

    // Main Menu button
    let button_menu_box_node = NodeBundle {
        style: Style {
            width: Val::Percent(80.0),
            height: Val::Percent(15.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            margin: UiRect::all(Val::Px(10.0)),
            ..default()
        },
        ..default()
    };
    let button_menu_node = ButtonBundle {
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
    let button_menu_text_node = TextBundle::from_section(
        "Main Menu",
        TextStyle {
            font_size: 56.0,
            font: textures.font.clone(),
            color: Color::srgb(0.9, 0.9, 0.9),
        },
    );

    // Exit button
    let button_exit_box_node = NodeBundle {
        style: Style {
            width: Val::Percent(80.0),
            height: Val::Percent(15.0),
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
            font_size: 56.0,
            font: textures.font.clone(),
            color: Color::srgb(0.9, 0.9, 0.9),
        },
    );

    let text_caption = commands.spawn(text_caption_node).id();
    let button_restart = commands
        .spawn(button_restart_box_node)
        .with_children(|parent| {
            parent
                .spawn((button_restart_node, ButtonType::Restart))
                .clear_children()
                .with_children(|parent| {
                    parent.spawn(button_restart_text_node);
                });
        })
        .id();
    let button_menu = commands
        .spawn(button_menu_box_node)
        .with_children(|parent| {
            parent
                .spawn((button_menu_node, ButtonType::MainMenu))
                .with_children(|parent| {
                    parent.spawn(button_menu_text_node);
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
    let padding_top = commands.spawn(padding_node.clone()).id();
    let padding_bottom = commands.spawn(padding_node).id();

    let main_menu = commands.spawn((main_menu_node, GameOverMenu {})).id();
    commands.entity(main_menu).push_children(&[
        padding_top,
        text_caption,
        button_restart,
        button_menu,
        button_exit,
        padding_bottom,
    ]);
}

#[allow(clippy::type_complexity)]
pub fn buttons_state_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &mut BorderColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color, mut border_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                border_color.0 = Color::BLACK;
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
                border_color.0 = Color::BLACK;
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
                ButtonType::Restart => next_state.set(GameState::Setup),
                ButtonType::MainMenu => next_state.set(GameState::Menu),
                ButtonType::Exit => {
                    std::process::exit(0);
                }
            }
        }
    }
}
