use bevy::prelude::Plugin;

use self::{
    resources::{HighScores, Score},
    systems::{handle_game_over, high_scores_updated, update_high_scores, update_score},
};

pub mod resources;
mod systems;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<Score>()
            .init_resource::<HighScores>()
            .add_system(update_score)
            .add_system(update_high_scores)
            .add_system(high_scores_updated)
            .add_system(handle_game_over);
    }
}
