use bevy::app::{App, Plugin};

mod actions;
mod components;
mod direction;
mod group;
mod positioning;
mod reactions;
mod signal;
mod target;

pub struct TrappedPlugin;

impl Plugin for TrappedPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            positioning::PositioningPlugin,
            reactions::ReactionsPlugin,
            signal::SignalPlugin,
        ));
    }
}
