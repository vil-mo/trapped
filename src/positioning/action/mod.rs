use crate::MakeStep;

use super::target::Target;
use bevy_ecs::world::World;
use enum_dispatch::enum_dispatch;
use std::collections::VecDeque;
use move_without_push::MoveWithoutPush;

pub mod move_without_push;

pub type Actions = VecDeque<ActionEnum>;

#[enum_dispatch]
pub trait Action: Copy {
    fn apply(self, target: Target, world: &mut World, actions: &mut Actions) -> MakeStep;
    fn undo(self, target: Target, world: &mut World);
}

#[enum_dispatch(Action)]
#[derive(Clone, Copy)]
pub enum ActionEnum {
    MoveWithoutPush,
}
