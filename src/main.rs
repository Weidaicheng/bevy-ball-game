mod events;
mod game;
mod main_menu;
mod states;
mod systems;

use bevy::{prelude::App, DefaultPlugins};
use game::GamePlugin;
use main_menu::MainMenuPlugin;
use states::AppState;
use systems::{
    exit_game, handle_game_over, spawn_camera, transition_to_game_state,
    transition_to_main_menu_state,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_state::<AppState>()
        .add_plugin(GamePlugin)
        .add_plugin(MainMenuPlugin)
        .add_system(spawn_camera)
        .add_system(transition_to_game_state)
        .add_system(transition_to_main_menu_state)
        .add_system(exit_game)
        .add_system(handle_game_over)
        .run();
}
