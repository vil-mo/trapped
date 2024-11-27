use std::time::Duration;

use bevy::prelude::Component;
use crate::actions::ActionEnum;

pub use crate::positioning::Object;

#[derive(Component)]
pub struct CurrentlyPerformedAction(ActionEnum);

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
