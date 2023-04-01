use bevy::prelude::{Commands, Input, KeyCode, NextState, Res, State};

use super::SimulationState;

pub fn toggle_simulation(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    simulation_state: Res<State<SimulationState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        if simulation_state.0 == SimulationState::Running {
            commands.insert_resource(NextState(Some(SimulationState::Paused)));
            println!("Simulation paused.");
        } else {
            commands.insert_resource(NextState(Some(SimulationState::Running)));
            println!("Simulation running.");
        }
    }
}
