use bevy::prelude::{IntoSystemAppConfig, IntoSystemConfigs, OnEnter, OnExit, OnUpdate, Plugin};

use crate::states::AppState;

use self::{
    resources::EnemySpawnTimer,
    systems::{
        confine_enemy_movement, despawn_enemy, enemy_movement, spawn_enemy, spawn_enemy_over_time,
        tick_enemy_spawn_timer, update_enemy_direction,
    },
};

use super::states::SimulationState;

pub mod components;
mod resources;
mod systems;

const NUMBER_OF_ENEMIES: usize = 4;
const ENEMY_SPEED: f32 = 200.0;
pub const ENEMY_SIZE: f32 = 64.0;
const ENEMY_SPAWN_TIME: f32 = 5.0;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<EnemySpawnTimer>()
            .add_system(spawn_enemy.in_schedule(OnEnter(AppState::Game)))
            .add_systems(
                (
                    enemy_movement,
                    update_enemy_direction,
                    confine_enemy_movement,
                    tick_enemy_spawn_timer,
                    spawn_enemy_over_time,
                )
                    .in_set(OnUpdate(AppState::Game))
                    .in_set(OnUpdate(SimulationState::Running)),
            )
            .add_system(despawn_enemy.in_schedule(OnExit(AppState::Game)));
    }
}
