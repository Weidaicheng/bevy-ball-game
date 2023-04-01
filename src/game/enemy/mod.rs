use bevy::prelude::Plugin;

use self::{
    resources::EnemySpawnTimer,
    systems::{
        confine_enemy_movement, enemy_movement, spawn_enemy, spawn_enemy_over_time,
        tick_enemy_spawn_timer, update_enemy_direction,
    },
};

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
            .add_startup_system(spawn_enemy)
            .add_system(enemy_movement)
            .add_system(update_enemy_direction)
            .add_system(confine_enemy_movement)
            .add_system(tick_enemy_spawn_timer)
            .add_system(spawn_enemy_over_time);
    }
}