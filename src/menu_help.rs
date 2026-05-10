use bevy::prelude::*;

use crate::{utils::{despawn_with_component, common_button_system}, constants::{BACKGROUND, TEXT_COLOR}, common_entity::spawn_button, GameState};


#[derive(Component)]
struct MenuHelpScreen;

#[derive(Component)]
enum GameOverMenuHelpButtonAction {
    Back,
}

pub struct MenuHelpPlugin;

impl Plugin for MenuHelpPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::HelpMenu), help_menu_setup)
            .add_systems(
                OnExit(GameState::HelpMenu),
                despawn_with_component::<MenuHelpScreen>,
            )
            .add_systems(Update, (menu_action, common_button_system));
    }
}

fn help_menu_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");

    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            BackgroundColor(Color::NONE),
            MenuHelpScreen,
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Node {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        padding: UiRect::px(20., 20., 10., 10.),
                        ..default()
                    },
                    BackgroundColor(BACKGROUND),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("CONTROLS"),
                        TextFont {
                            font: font.clone(),
                            font_size: 60.0,
                            ..default()
                        },
                        TextColor(TEXT_COLOR),
                        Node {
                            margin: UiRect::all(Val::Px(10.0)),
                            ..default()
                        },
                    ));

                    let game_score = format!("
                    Left : move left    \n
                    Right : move right                 \n
                    Up : rotate \n
                    Down : soft drop     \n
                    Space : hard drop      \n
                    Esc : pause game       \n");
                    parent.spawn((
                        Text::new(game_score),
                        TextFont {
                            font: font.clone(),
                            font_size: 40.0,
                            ..default()
                        },
                        TextColor(TEXT_COLOR),
                        Node {
                            margin: UiRect::all(Val::Px(0.0)),
                            ..default()
                        },
                    ));

                    spawn_button(parent, GameOverMenuHelpButtonAction::Back, "right.png", "Back", &asset_server);
                });
        });
}

fn menu_action(
    interaction_query: Query<
        (&Interaction, &GameOverMenuHelpButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button_action {
                GameOverMenuHelpButtonAction::Back => {
                    game_state.set(GameState::Menu);
                }
            }
        }
    }
}
