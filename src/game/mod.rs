mod enemy;
mod player;
mod score;
mod star;
pub mod states;
mod systems;

use bevy::prelude::{in_state, IntoSystemConfig, Plugin};

use crate::{events::GameOver, AppState};

use self::{
    enemy::EnemyPlugin, player::PlayerPlugin, score::ScorePlugin, star::StarPlugin,
    states::SimulationState, systems::toggle_simulation,
};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<GameOver>()
            .add_state::<SimulationState>()
            .add_plugin(EnemyPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(ScorePlugin)
            .add_plugin(StarPlugin)
            .add_system(toggle_simulation.run_if(in_state(AppState::Game)));
    }
}
