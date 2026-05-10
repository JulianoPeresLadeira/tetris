use bevy::prelude::*;

use crate::{
    common_entity::spawn_button,
    constants::{BACKGROUND, TEXT_COLOR},
    utils::{common_button_system, despawn_with_component},
    GameState,
};

pub struct MenuPlugin;

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum GameSelectedLevel {
    Easy,
    Normal,
    Hard,
}

#[derive(Resource)]
pub struct GameLevelRes(pub GameSelectedLevel);

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameLevelRes(GameSelectedLevel::Easy))
            .add_systems(OnEnter(GameState::Menu), main_menu_setup)
            .add_systems(
                OnExit(GameState::Menu),
                despawn_with_component::<OnMainMenuScreen>,
            )
            .add_systems(Update, (menu_action, common_button_system));
    }
}

#[derive(Component)]
struct OnMainMenuScreen;

#[derive(Component)]
enum MenuButtonAction {
    EasyPlay,
    NormalPlay,
    HardPlay,
    Help,
    Quit,
}

fn main_menu_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
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
            OnMainMenuScreen,
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Node {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        padding: UiRect::px(120., 120., 10., 30.),
                        ..default()
                    },
                    BackgroundColor(BACKGROUND),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("TETRIS"),
                        TextFont {
                            font: font.clone(),
                            font_size: 80.0,
                            ..default()
                        },
                        TextColor(TEXT_COLOR),
                        Node {
                            margin: UiRect::all(Val::Px(50.0)),
                            ..default()
                        },
                    ));

                    spawn_button(parent, MenuButtonAction::EasyPlay, "right.png", "Easy", &asset_server);
                    spawn_button(parent, MenuButtonAction::NormalPlay, "right.png", "Normal", &asset_server);
                    spawn_button(parent, MenuButtonAction::HardPlay, "right.png", "Hard", &asset_server);
                    spawn_button(parent, MenuButtonAction::Help, "wrench.png", "How To Play", &asset_server);
                    spawn_button(parent, MenuButtonAction::Quit, "exitRight.png", "Quit", &asset_server);
                });
        });
}

#[allow(unused_mut)]
#[allow(unused_variables)]
fn menu_action(
    interaction_query: Query<
        (&Interaction, &MenuButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut app_exit_events: ResMut<Messages<AppExit>>,
    mut game_state: ResMut<NextState<GameState>>,
    mut game_level: ResMut<GameLevelRes>,
) {
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button_action {
                MenuButtonAction::Quit =>
                {
                    app_exit_events.write(AppExit::Success);
                }
                MenuButtonAction::EasyPlay => {
                    game_state.set(GameState::Game);
                    game_level.0 = GameSelectedLevel::Easy;
                }
                MenuButtonAction::NormalPlay => {
                    game_state.set(GameState::Game);
                    game_level.0 = GameSelectedLevel::Normal;
                }
                MenuButtonAction::HardPlay => {
                    game_state.set(GameState::Game);
                    game_level.0 = GameSelectedLevel::Hard;
                }
                MenuButtonAction::Help => {
                    game_state.set(GameState::HelpMenu);
                }
            }
        }
    }
}
