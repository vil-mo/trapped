use crate::{signal::Signal, target::TargetedAction};
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

type SignalReaction = fn(Signal, &World) -> Vec<TargetedAction>;

#[derive(Resource, Default)]
struct SignalReactions(Vec<SignalReaction>);

type StepReaction = fn(&World) -> Vec<TargetedAction>;

#[derive(Resource, Default)]
struct StepReactions(Vec<StepReaction>);
