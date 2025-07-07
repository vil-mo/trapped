use crate::level_state::LevelState;
use enum_dispatch::enum_dispatch;

pub mod push;
// pub mod willing_move;

pub struct ActionResult {
    pub further_actions: Vec<ActionEnum>,
}

impl Default for ActionResult {
    fn default() -> Self {
        Self {
            further_actions: Vec::new(),
        }
    }
}

#[enum_dispatch]
pub trait Action: Into<ActionEnum> {
    fn apply(&self, level_state: &LevelState) -> ActionResult;
}

#[derive(Clone)]
pub struct NoAction;

impl Action for NoAction {
    fn apply(&self, _: &LevelState) -> ActionResult {
        ActionResult::default()
    }
}

#[enum_dispatch(Action)]
pub enum ActionEnum {
    NoAction(NoAction),
    // WillingMove(willing_move::WillingMove),
}
