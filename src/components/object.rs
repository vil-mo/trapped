use std::time::Duration;
use crate::actions::{ActionEnum, ActionIsBeing};
use bevy::prelude::Component;

pub struct RegisterObjectComponentsPlugin;

impl bevy::app::Plugin for RegisterObjectComponentsPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        let world = app.world_mut();
        world.register_component::<Object>();
        world.register_component::<CurrentlyPerformedAction>();
        world.register_component::<ActionsDuration>();
        world.register_component::<NeedsWalkableFloor>();
        world.register_component::<PassesThroughWalls>();
    }
}

pub use crate::positioning::Object;

#[derive(Component)]
pub struct CurrentlyPerformedAction(pub ActionEnum, pub ActionIsBeing);

#[derive(Component)]
pub struct ActionsDuration {
    pub movement: Duration,
    pub push: Duration,
}

/// Object only can move if in the position it moves onto there is a
/// [`Floor`](super::floor::Floor) that is not [`Unwalkable`](super::floor::Unwalkable).
#[derive(Component)]
pub struct NeedsWalkableFloor;

/// Object can pass through walls even if it's not [`AlwaysPassable`](super::wall::AlwaysPassable).
#[derive(Component)]
pub struct PassesThroughWalls;
