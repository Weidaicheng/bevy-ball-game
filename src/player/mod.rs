use bevy::prelude::{Plugin, App};

use self::systems::{
    confine_player_movement, enemy_hit_player, player_hit_star, player_movement, spawn_player,
};

mod components;
mod systems;

const PLAYER_SIZE: f32 = 64.0;
const PLAYER_SPEED: f32 = 500.0;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player)
            .add_system(player_movement)
            .add_system(confine_player_movement)
            .add_system(enemy_hit_player)
            .add_system(player_hit_star);
    }
}
