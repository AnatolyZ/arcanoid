use crate::brick::Brick;
use crate::textures::resources::Textures;
use bevy::prelude::*;

use super::components::LevelCompleteMenu;
use crate::states::GameState;

pub fn spawn_level_complete_menu(textures: Res<Textures>, mut commands: Commands) {
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

    // vertical container
    let vertical_container = NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            display: Display::Flex,
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::SpaceAround,
            align_items: AlignItems::Center,
            ..Default::default()
        },
        ..Default::default()
    };

    // Caption
    let text_caption_node = TextBundle {
        text: Text {
            sections: vec![
                TextSection {
                    value: "Level Complete!\n".to_string(),
                    style: TextStyle {
                        font: textures.font.clone(),
                        font_size: 140.0,
                        color: Color::srgb(0.0, 1.0, 0.0),
                    },
                },
                TextSection {
                    value: "Press ".to_string(),
                    style: TextStyle {
                        font: textures.font.clone(),
                        font_size: 100.0,
                        color: Color::BLACK,
                    },
                },
                TextSection {
                    value: "SPACE".to_string(),
                    style: TextStyle {
                        font: textures.font.clone(),
                        font_size: 100.0,
                        color: Color::srgb(0.0, 1.0, 0.0),
                    },
                },
                TextSection {
                    value: " to continue".to_string(),
                    style: TextStyle {
                        font: textures.font.clone(),
                        font_size: 100.0,
                        color: Color::BLACK,
                    },
                },
            ],
            justify: JustifyText::Center,
            linebreak_behavior: bevy::text::BreakLineOn::NoWrap,
        },
        style: Style {
            margin: UiRect::all(Val::Px(10.0)),
            ..Default::default()
        },
        ..Default::default()
    };

    let text_node = commands
        .spawn(vertical_container)
        .with_children(|parent| {
            parent.spawn(text_caption_node);
        })
        .id();
    let main_menu = commands.spawn((main_menu_node, LevelCompleteMenu {})).id();
    commands.entity(main_menu).push_children(&[text_node]);
}

pub fn despawn_level_complete_menu(
    mut commands: Commands,
    level_complete_menu_query: Query<Entity, With<LevelCompleteMenu>>,
) {
    for entity in level_complete_menu_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn wait_for_key_press(
    mut next_state: ResMut<NextState<GameState>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        next_state.set(GameState::Setup);
    }
}

pub fn check_for_completion(
    brick_query: Query<&Brick>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if brick_query.iter().count() == 0 {
        next_state.set(GameState::LevelComplete);
    }
}
