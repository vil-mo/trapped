use crate::level_state::StateChangeEnum;
use bevy::ecs::world::World;
use enum_dispatch::enum_dispatch;

pub mod force_move;
pub mod push;
// pub mod willing_move;

pub enum ActionResult {
    MoreActions(Vec<ActionEnum>),
    StateChange(StateChangeEnum),
}

impl Default for ActionResult {
    fn default() -> Self {
        Self::MoreActions(Vec::new())
    }
}

pub enum ActionIsBeing {
    Applied,
    Undone,
}

#[enum_dispatch]
pub trait Action: Copy {
    fn apply(self, world: &mut World) -> ActionResult;
}

#[derive(Clone, Copy)]
pub struct NoAction;

impl Action for NoAction {
    fn apply(self, _world: &mut World) -> ActionResult {
        ActionResult::default()
    }
}

#[enum_dispatch(Action)]
#[derive(Clone, Copy)]
pub enum ActionEnum {
    NoAction(NoAction),
    // WillingMove(willing_move::WillingMove),
}
