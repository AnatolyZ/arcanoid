use crate::main_menu::components::MainMenu;
use bevy::prelude::*;

pub fn spawn_main_menu(mut commands: Commands) {
    let main_menu_entity = build_main_meny(commands);
}

pub fn despawn_main_menu(mut commands: Commands, main_menu_query: Query<Entity, With<MainMenu>>) {
    for entity in main_menu_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn build_main_meny(mut commands: Commands) -> Entity {
    let main_menu_entity = commands
        .spawn((
            NodeBundle {
                background_color: Color::RED.into(),
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            MainMenu {},
        ))
        .id();
    main_menu_entity
}
