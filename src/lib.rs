//! Correctness formalization if I were to give someone else development rights:
//! - (State of the board)[actions::Action::undo]
//! - (Cells occupation)[positioning::movement]

use bevy::app::{App, Plugin};

mod actions;
mod components;
mod direction;
mod game_loop;
mod group;
mod positioning;
mod signal;
mod level_state;
mod target;

pub struct TrappedPlugin;

impl Plugin for TrappedPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            positioning::PositioningPlugin,
            game_loop::GameLoopPlugin,
            signal::SignalPlugin,
        ));
    }
}
