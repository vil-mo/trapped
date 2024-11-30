use crate::direction::Direction;
use bevy::prelude::Event;

pub struct SignalPlugin;

impl bevy::app::Plugin for SignalPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_event::<Signal>();
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Event)]
pub enum Signal {
    Move(Direction),
}
