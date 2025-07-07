use crate::{
    components::object::{ActionsDuration, CurrentlyPerformedAction},
    direction::Direction,
    positioning::movement::{can_move, move_target, CanMove},
    target::Target,
};
use bevy::prelude::World;
use std::time::Duration;

use super::{Action, ActionEnum, ActionIsBeing, ActionStatus};

#[derive(Clone, Copy)]
pub struct WillingMove(pub Direction);

impl Action for WillingMove {
    fn apply(self, target: Target, world: &mut World) -> ActionStatus {
        let can_move = can_move(world, target, self.0);

        if let CanMove::Can = can_move {
            // CORRECTNESS: `can_move` returns `CanMove::Can`
            move_target(world, target, self.0);

            let mut duration = Duration::ZERO;
            let fitting_objects = target.fitting_objects(world);
            for object in fitting_objects {
                let mut object = world.entity_mut(object);
                duration = duration.max(
                    object
                        .get::<ActionsDuration>()
                        .expect("`ActionsDuration` should be added with `Object` component")
                        .movement,
                );
                if let Some(mut currently_performed) = object.get_mut::<CurrentlyPerformedAction>()
                {
                    currently_performed.0 = ActionEnum::WillingMove(self);
                    currently_performed.1 = ActionIsBeing::Applied;
                }
            }

            if duration != Duration::ZERO {
                return ActionStatus::Made(duration, vec![]);
            }
        }
        ActionStatus::Failed
    }

    fn undo(self, target: Target, world: &mut World) {
        move_target(world, target, !self.0);
        let fitting_objects = target.fitting_objects(world);
        for object in fitting_objects {
            let mut object = world.entity_mut(object);
            if let Some(mut currently_performed) = object.get_mut::<CurrentlyPerformedAction>() {
                currently_performed.0 = ActionEnum::WillingMove(self);
                currently_performed.1 = ActionIsBeing::Undone;
            }
        }
    }
}
