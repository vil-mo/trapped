use crate::signal::Signal;
use bevy::{
    ecs::schedule::SystemConfigs,
    prelude::{EventReader, IntoSystemConfigs, ResMut, Resource},
};
use std::collections::VecDeque;

pub struct SignalQueuePlugin;

impl bevy::app::Plugin for SignalQueuePlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.init_resource::<SignalQueue>();
    }
}

#[derive(Resource, Default)]
pub struct SignalQueue(VecDeque<Signal>);

impl SignalQueue {
    pub fn push(&mut self, signal: Signal) {
        self.0.push_back(signal);
    }

    pub fn pop(&mut self) -> Option<Signal> {
        self.0.pop_front()
    }
}

pub fn update_signal_queue() -> SystemConfigs {
    update_queue.into_configs()
}

fn update_queue(mut signals: EventReader<Signal>, mut queue: ResMut<SignalQueue>) {
    for signal in signals.read() {
        queue.push(*signal);
    }
}
