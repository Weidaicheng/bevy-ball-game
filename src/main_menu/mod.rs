use bevy::prelude::Plugin;

use self::systems::spawn_main_menu;

mod systems;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_startup_system(spawn_main_menu);
    }
}
