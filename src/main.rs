mod camera;
mod demo;
mod enemy;
mod game;
mod player;
mod score;
mod star;

use bevy::{prelude::App, DefaultPlugins};
use camera::CameraPlugin;
use enemy::EnemyPlugin;
use game::GamePlugin;
use player::PlayerPlugin;
use score::ScorePlugin;
use star::StarPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(CameraPlugin)
        .add_plugin(GamePlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(EnemyPlugin)
        .add_plugin(StarPlugin)
        .add_plugin(ScorePlugin)
        .run();
}
