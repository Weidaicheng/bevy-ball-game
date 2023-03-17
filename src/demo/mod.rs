use bevy::prelude::{App, Plugin};

use self::systems::{move_ground_left_over_time, spawn_ground};

mod components;
mod systems;

const GROUND_WIDTH: f32 = 808.0;
const GROUND_HEIGHT: f32 = 71.0;
const GROUND_MOVE_SPEED: f32 = 100.0;

pub struct DemoPlugin;

impl Plugin for DemoPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_ground)
            .add_system(move_ground_left_over_time);
    }
}
