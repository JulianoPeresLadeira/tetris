use bevy::{
    asset::{AssetMetaCheck, AssetPlugin}, core_pipeline::core_2d::Camera2dBundle, prelude::{default, App, ClearColor, Commands, PluginGroup, Startup, States}, state::app::AppExtStates, window::{PresentMode, Window, WindowPlugin, WindowResolution}, DefaultPlugins
};

use constants::{WINDOW_HEIGHT, WINDOW_WIDTH, BACKGROUND_COLOR};

mod board;
mod brick;
mod constants;
mod menu;
mod position;
mod utils;
mod game;
mod gameover;
mod common_entity;
mod data;
mod menu_help;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum GameState {
    #[default]
    Menu,
    Game,
    GameOver,
    HelpMenu
}

fn main() {
    App::new()
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        //.insert_resource(AssetMetaCheck::Never)
        .add_plugins(DefaultPlugins
            .set(AssetPlugin {
                meta_check: AssetMetaCheck::Never,
                ..default()
            })
            .set(WindowPlugin {
            primary_window: Some(Window {
                title: "TETRIS".to_string(),
                resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT),
                present_mode: PresentMode::AutoVsync,
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        // .add_plugins(WorldInspectorPlugin::new())
        .add_systems(Startup, camera_setup)
        .init_state::<GameState>()
        .add_plugins(menu::MenuPlugin)
        .add_plugins(game::GamePlugin)
        .add_plugins(gameover::GameOverPlugin)
        .add_plugins(menu_help::MenuHelpPlugin)
        .run();
}

fn camera_setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
