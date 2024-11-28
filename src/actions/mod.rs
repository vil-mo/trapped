use crate::target::{Target, TargetedAction};
use bevy::ecs::world::World;
use enum_dispatch::enum_dispatch;
use std::time::Duration;

pub mod force_move;
pub mod push;
pub mod willing_move;

pub enum ActionStatus {
    Made(Duration, Vec<TargetedAction>),
    InstantlyMade,
    Failed,
}

impl ActionStatus {
    pub fn is_failed(&self) -> bool {
        matches!(self, ActionStatus::Failed)
    }
}

pub enum ActionIsBeing {
    Applied,
    Undone,
}

#[enum_dispatch]
pub trait Action: Copy {
    fn apply(self, target: Target, world: &mut World) -> ActionStatus;

    /// CORRECTNESS: Should only be called if this action was applied and
    /// the state of the board is the same as it was right after this exact action was applied.
    fn undo(self, target: Target, world: &mut World);
}

#[derive(Clone, Copy)]
pub struct NoAction;

impl Action for NoAction {
    fn apply(self, _target: Target, _world: &mut World) -> ActionStatus {
        ActionStatus::InstantlyMade
    }
    fn undo(self, _target: Target, _world: &mut World) {}
}

#[enum_dispatch(Action)]
#[derive(Clone, Copy)]
pub enum ActionEnum {
    NoAction(NoAction),
    WillingMove(willing_move::WillingMove),
}
