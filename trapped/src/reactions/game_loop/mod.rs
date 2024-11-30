use crate::actions::ActionStatus;

use super::{SignalReactions, StepReactions};
use action_queue::ActionsQueue;
use bevy::{
    app::Update,
    prelude::{IntoSystemConfigs, Mut, World},
};
use executing_action_timer::ExecutingActionTimer;
use signal_queue::{update_signal_queue, SignalQueue};

mod action_queue;
mod executing_action_timer;
mod signal_queue;

pub struct GameLoopPlugin;

impl bevy::app::Plugin for GameLoopPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_plugins((
            action_queue::ActionsQueuePlugin,
            executing_action_timer::ExecutingActionTimerPlugin,
            signal_queue::SignalQueuePlugin,
        ));
        app.add_systems(Update, update_signal_queue().before(game_loop_system));
    }
}

fn process_signal(world: &mut World) {
    world.resource_scope(|world, mut queue: Mut<ActionsQueue>| {
        queue.reset();

        let Some(signal) = world.resource_mut::<SignalQueue>().pop() else {
            return;
        };

        world.resource_scope(|world, signal_reactions: Mut<SignalReactions>| {
            for reaction in signal_reactions.0.iter() {
                let actions = reaction(signal, world);
                queue.extend(actions);
            }
        });
    });
}

fn process_step(world: &mut World) {
    world.resource_scope(|world, mut queue: Mut<ActionsQueue>| {
        queue.reset_with_processing_step();

        world.resource_scope(|world, step_reactions: Mut<StepReactions>| {
            for reaction in step_reactions.0.iter() {
                let actions = reaction(world);
                queue.extend(actions);
            }
        });
    });
}

fn game_loop_system(world: &mut World) {
    while !world.resource::<ExecutingActionTimer>().executing() {
        world.resource_scope(|world, mut actions_queue: Mut<ActionsQueue>| {
            let status = actions_queue.pop_front(world);

            if let Some(status) = status {
                if let ActionStatus::Made(duration, additional_actions) = status {
                    world.resource_mut::<ExecutingActionTimer>().start(duration);
                    actions_queue.extend(additional_actions);
                }
            } else if actions_queue.should_make_step() {
                process_step(world);
            } else {
                process_signal(world);
            }
        });
    }
}
