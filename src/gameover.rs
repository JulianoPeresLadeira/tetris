use bevy::prelude::*;

use crate::{utils::{despawn_with_component, common_button_system}, constants::{BACKGROUND, TEXT_COLOR}, common_entity::spawn_button, GameState, game::GameScoresRes};


#[derive(Component)]
struct GameOverMenuScreen;

#[derive(Component)]
enum GameOverMenuButtonAction {
    Back,
    Quit,
}

pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::GameOver), gameover_menu_setup)
            .add_systems(
                OnExit(GameState::GameOver),
                despawn_with_component::<GameOverMenuScreen>,
            )
            .add_systems(Update, (menu_action, common_button_system));
    }
}

fn gameover_menu_setup(mut commands: Commands, asset_server: Res<AssetServer>, game_scores_stored: Res<GameScoresRes>) {
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
            GameOverMenuScreen,
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Node {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        padding: UiRect::px(20., 20., 10., 30.),
                        ..default()
                    },
                    BackgroundColor(BACKGROUND),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("GAME OVER"),
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

                    let game_score = format!("Score : {:}   Level : {:}   Lines : {:}", game_scores_stored.score, game_scores_stored.level, game_scores_stored.lines);
                    parent.spawn((
                        Text::new(game_score),
                        TextFont {
                            font: font.clone(),
                            font_size: 30.0,
                            ..default()
                        },
                        TextColor(TEXT_COLOR),
                        Node {
                            margin: UiRect::all(Val::Px(20.0)),
                            ..default()
                        },
                    ));

                    spawn_button(parent, GameOverMenuButtonAction::Back, "right.png", "Main Menu", &asset_server);
                    spawn_button(parent, GameOverMenuButtonAction::Quit, "exitRight.png", "Quit", &asset_server);
                });
        });
}

#[allow(unused_mut)]
#[allow(unused_variables)]
fn menu_action(
    interaction_query: Query<
        (&Interaction, &GameOverMenuButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut app_exit_events: ResMut<Messages<AppExit>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button_action {
                GameOverMenuButtonAction::Quit => {
                    app_exit_events.write(AppExit::Success);
                }
                GameOverMenuButtonAction::Back => {
                    game_state.set(GameState::Menu);
                }
            }
        }
    }
}
