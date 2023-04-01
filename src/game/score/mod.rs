use bevy::prelude::{in_state, IntoSystemAppConfig, IntoSystemConfig, OnEnter, OnExit, Plugin};

use crate::states::AppState;

use self::{
    resources::HighScores,
    systems::{high_scores_updated, insert_score, remove_score, update_high_scores, update_score},
};

pub mod resources;
mod systems;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<HighScores>()
            .add_system(insert_score.in_schedule(OnEnter(AppState::Game)))
            .add_system(update_score.run_if(in_state(AppState::Game)))
            .add_system(update_high_scores)
            .add_system(high_scores_updated)
            .add_system(remove_score.in_schedule(OnExit(AppState::Game)));
    }
}
