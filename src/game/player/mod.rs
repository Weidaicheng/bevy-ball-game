use bevy::prelude::{
    App, IntoSystemAppConfig, IntoSystemConfigs, OnEnter, OnExit, OnUpdate, Plugin,
};

use crate::states::AppState;

use self::systems::{
    confine_player_movement, despawn_player, enemy_hit_player, player_hit_star, player_movement,
    spawn_player,
};

use super::states::SimulationState;

mod components;
mod systems;

const PLAYER_SIZE: f32 = 64.0;
const PLAYER_SPEED: f32 = 500.0;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_player.in_schedule(OnEnter(AppState::Game)))
            .add_systems(
                (
                    player_movement,
                    confine_player_movement,
                    enemy_hit_player,
                    player_hit_star,
                )
                    .in_set(OnUpdate(AppState::Game))
                    .in_set(OnUpdate(SimulationState::Running)),
            )
            .add_system(despawn_player.in_schedule(OnExit(AppState::Game)));
    }
}
