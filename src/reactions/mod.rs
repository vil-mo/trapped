use crate::{positioning::action::target::TargetedAction, signal::Signal};
use bevy::prelude::{Resource, World};

mod game_loop;

pub struct ReactionsPlugin;

impl bevy::app::Plugin for ReactionsPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.init_resource::<SignalReactions>();
        app.init_resource::<StepReactions>();
        app.add_plugins(game_loop::GameLoopPlugin);
    }
}

type SignalReaction = fn(Signal, &mut World) -> Vec<TargetedAction>;

#[derive(Resource)]
struct SignalReactions(Vec<SignalReaction>);

impl Default for SignalReactions {
    fn default() -> Self {
        Self(vec![])
    }
}

type StepReaction = fn(&mut World) -> Vec<TargetedAction>;

#[derive(Resource)]
struct StepReactions(Vec<StepReaction>);

impl Default for StepReactions {
    fn default() -> Self {
        Self(vec![])
    }
}
