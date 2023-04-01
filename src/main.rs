mod events;
mod game;
mod main_menu;
mod systems;

use bevy::{prelude::App, DefaultPlugins};
use events::GameOver;
use game::GamePlugin;
use main_menu::MainMenuPlugin;
use systems::{exit_game, handle_game_over, spawn_camera};

fn main() {
    App::new()
        .add_event::<GameOver>()
        .add_plugins(DefaultPlugins)
        .add_plugin(GamePlugin)
        .add_plugin(MainMenuPlugin)
        .add_system(spawn_camera)
        .add_system(exit_game)
        .add_system(handle_game_over)
        .run();
}
