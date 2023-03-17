use bevy::prelude::Plugin;

use self::{events::GameOver, systems::exit_game};

pub mod events;
mod systems;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<GameOver>().add_system(exit_game);
    }
}
