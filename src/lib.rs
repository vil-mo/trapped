//! Correctness formalization if I were to develop with someone else:
//! - (State of the board)[actions::Action::undo]
//! - (Cells occupation)[positioning::movement]

#[warn(clippy::all)]

use bevy::app::{App, Plugin};

mod action;
mod component;
mod direction;
mod game_loop;
mod level_state;

pub struct TrappedPlugin;

impl Plugin for TrappedPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            component::RegisterComponentsPlugin,
            game_loop::GameLoopPlugin,
        ));
    }
}
