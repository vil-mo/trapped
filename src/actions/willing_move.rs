use std::time::Duration;

use bevy::prelude::{Entity, World};

use crate::{
    components::object::ActionsDuration,
    direction::Direction,
    positioning::{movement::translate, Object},
    target::Target,
};

use super::{Action, ActionStatus};

#[derive(Clone, Copy)]
pub struct WillingMove(pub Direction);

impl Action for WillingMove {
    fn apply(self, target: Target, world: &mut World) -> ActionStatus {
        let mut duration = Duration::ZERO;


        ActionStatus::Made(duration, vec![])
    }

    fn undo(self, target: Target, world: &mut World) {}
}
