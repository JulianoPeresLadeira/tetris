use bevy::prelude::*;
use bevy::ecs::relationship::RelatedSpawnerCommands;
use bevy::ecs::hierarchy::ChildOf;

use crate::constants::{TEXT_COLOR, NORMAL_BUTTON};

pub fn spawn_button(
    parent: &mut RelatedSpawnerCommands<'_, ChildOf>,
    action: impl Component,
    icon_image_path: &'static str,
    title: &str,
    asset_server: &AssetServer,
) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let icon = asset_server.load(icon_image_path);

    parent.spawn((
        Button,
        Node {
            width: Val::Px(250.0),
            height: Val::Px(65.0),
            margin: UiRect::all(Val::Px(20.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(NORMAL_BUTTON),
        action,
    )).with_children(|parent| {
        parent.spawn((
            ImageNode::new(icon),
            Node {
                width: Val::Px(30.0),
                height: Val::Auto,
                ..default()
            },
        ));
        parent.spawn((
            Text::new(title),
            TextFont {
                font: font.clone(),
                font_size: 40.0,
                ..default()
            },
            TextColor(TEXT_COLOR),
        ));
    });
}
