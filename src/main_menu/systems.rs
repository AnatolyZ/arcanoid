use crate::main_menu::components::MainMenu;
use crate::textures::resources::Textures;
use bevy::prelude::*;

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const PRESSED_BUTTON: Color = Color::rgb(0.1, 0.1, 0.1);
const HOVERED_BUTTON: Color = Color::rgb(0.2, 0.2, 0.2);

pub fn spawn_main_menu(mut commands: Commands, textures: Res<Textures>) {
    let main_menu_entity = build_main_menu(commands, textures);
}

pub fn despawn_main_menu(mut commands: Commands, main_menu_query: Query<Entity, With<MainMenu>>) {
    for entity in main_menu_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn build_main_menu(mut commands: Commands, textures: Res<Textures>) -> Entity {
    // Main menu container
    let main_menu_node = NodeBundle {
        background_color: Color::hex("#7f8a88").unwrap().into(),
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Start,
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
                ..Default::default()
            }],
            alignment: TextAlignment::Center,
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
            color: Color::rgb(0.9, 0.9, 0.9),
            ..default()
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
            color: Color::rgb(0.9, 0.9, 0.9),
            ..default()
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
            color: Color::rgb(0.9, 0.9, 0.9),
            ..default()
        },
    );

    let text_caption = commands.spawn(text_caption_node).id();
    let main_menu = commands.spawn((main_menu_node, MainMenu {})).id();
    let button_start = commands
        .spawn(button_start_box_node)
        .with_children(|parent| {
            parent
                .spawn(button_start_node)
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
            parent.spawn(button_exit_node).with_children(|parent| {
                parent.spawn(button_exit_text_node);
            });
        })
        .id();

    commands.entity(main_menu).push_children(&[
        text_caption,
        button_start,
        button_options,
        button_exit,
    ]);

    main_menu
}

pub fn button_system(
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
                border_color.0 = Color::RED;
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
