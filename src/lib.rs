use std::{
    collections::VecDeque,
    io::Take,
    ops::{BitAnd, BitOr},
};

use bevy_ecs::{
    component::Component,
    entity::Entity,
    event::{Event, EventCursor, Events},
    system::{BoxedSystem, In, Local, Resource},
    world::{Mut, World},
};
use direction::Direction;
use positioning::{
    action::{Action, ActionEnum},
    target::Target,
};

pub mod direction;
pub mod positioning;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Event)]
pub enum Signal {
    Move(Direction),
    Step,
}

pub enum TakeAction {
    None,
    Take(ActionEnum, Target),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Group {
    None(Entity),
    Red,
    Blue,
    Green,
    Yellow,
    Pink,
    Cyan,
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum GroupComponent {
    #[default]
    None,
    Red,
    Blue,
    Green,
    Yellow,
    Pink,
    Cyan,
}

impl Group {
    pub fn from_component(component: Option<GroupComponent>, entity: Entity) -> Self {
        match component {
            Some(GroupComponent::None) => Group::None(entity),
            Some(GroupComponent::Red) => Group::Red,
            Some(GroupComponent::Blue) => Group::Blue,
            Some(GroupComponent::Green) => Group::Green,
            Some(GroupComponent::Yellow) => Group::Yellow,
            Some(GroupComponent::Pink) => Group::Pink,
            Some(GroupComponent::Cyan) => Group::Cyan,
            None => Group::None(entity),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MakeStep {
    Make,
    DontMake,
}

impl MakeStep {
    #[inline]
    fn to_bool(self) -> bool {
        match self {
            MakeStep::Make => true,
            MakeStep::DontMake => false,
        }
    }
}

impl BitOr for MakeStep {
    type Output = Self;

    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        if self.to_bool() || rhs.to_bool() {
            MakeStep::Make
        } else {
            MakeStep::DontMake
        }
    }
}

impl BitAnd for MakeStep {
    type Output = Self;

    #[inline]
    fn bitand(self, rhs: Self) -> Self::Output {
        if self.to_bool() && rhs.to_bool() {
            MakeStep::Make
        } else {
            MakeStep::DontMake
        }
    }
}

#[derive(Resource)]
pub struct ProcessSingnalsPasses {
    signal_reactions: Vec<BoxedSystem<In<Signal>, TakeAction>>,
}

fn process_signals(world: &mut World, mut signal_cursor: Local<EventCursor<Signal>>) {
    world.resource_scope(|world, mut passes: Mut<ProcessSingnalsPasses>| {
        let mut signals = VecDeque::from_iter(
            signal_cursor
                .read(world.resource::<Events<Signal>>())
                .copied(),
        );

        while let Some(signal) = signals.pop_front() {
            let mut make_step = MakeStep::DontMake;

            for pass in &mut passes.signal_reactions {
                let take_action = pass.run(signal, world);
                
                if let TakeAction::Take(action, target) = take_action {
                    let mut actions = VecDeque::new();
                    actions.push_back(action);
                    while let Some(action) = actions.pop_front() {
                        make_step = make_step | action.apply(target, world, &mut actions);
                    }
                }
            }

            if MakeStep::Make == make_step {
                signals.push_front(Signal::Step);
            }
        }
    });
}
