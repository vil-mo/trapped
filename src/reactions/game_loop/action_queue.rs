use bevy::prelude::{Resource, World};
use std::{collections::VecDeque, ops::BitOrAssign};

use crate::{actions::ActionStatus, target::TargetedAction};

pub struct ActionsQueuePlugin;

impl bevy::app::Plugin for ActionsQueuePlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.init_resource::<ActionsQueue>();
    }
}

#[derive(Clone, Copy, Default)]
enum DesidingToMakeStepVariants {
    #[default]
    DontMake,
    Make,
    ProcessingStep,
}

impl BitOrAssign<bool> for DesidingToMakeStepVariants {
    fn bitor_assign(&mut self, rhs: bool) {
        match (*self, rhs) {
            (DesidingToMakeStepVariants::DontMake, true) => {
                *self = DesidingToMakeStepVariants::Make;
            }
            _ => (),
        }
    }
}

/// Actions that vere queued in from the signal.
#[derive(Resource, Default)]
pub struct ActionsQueue {
    queue: VecDeque<TargetedAction>,
    should_make_step: DesidingToMakeStepVariants,
}

impl ActionsQueue {
    pub fn pop_front(&mut self, world: &mut World) -> Option<ActionStatus> {
        self.queue.pop_front().map(|action| {
            let status = action.apply(world);
            self.should_make_step |= !status.is_failed();
            status
        })
    }

    pub fn should_make_step(&self) -> bool {
        matches!(self.should_make_step, DesidingToMakeStepVariants::Make)
    }

    pub fn reset(&mut self) {
        self.queue.clear();
        self.should_make_step = DesidingToMakeStepVariants::DontMake;
    }

    pub fn reset_with_processing_step(&mut self) {
        self.queue.clear();
        self.should_make_step = DesidingToMakeStepVariants::ProcessingStep;
    }
}

impl Extend<TargetedAction> for ActionsQueue {
    fn extend<T: IntoIterator<Item = TargetedAction>>(&mut self, iter: T) {
        self.queue.extend(iter)
    }
}
