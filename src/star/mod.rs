use bevy::prelude::Plugin;

use self::{
    resources::StarSpawnTimer,
    systems::{spawn_stars, spawn_stars_over_time, tick_star_spawn_timer},
};

pub mod components;
mod resources;
mod systems;

const NUMBER_OF_STARS: usize = 10;
pub const STAR_SIZE: f32 = 30.0;
const STAR_SPAWN_TIME: f32 = 1.0;

pub struct StarPlugin;

impl Plugin for StarPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<StarSpawnTimer>()
            .add_startup_system(spawn_stars)
            .add_system(tick_star_spawn_timer)
            .add_system(spawn_stars_over_time);
    }
}