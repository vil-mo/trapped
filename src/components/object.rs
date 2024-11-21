use bevy::prelude::Component;

pub use crate::positioning::Object;

/// Object only can move if in the position it moves onto there is a
/// [`Floor`](super::floor::Floor) that is not [`Unwalkable`](super::floor::Unwalkable).
#[derive(Component)]
pub struct NeedsWalkableFloor;

/// Object can pass through walls even if it's not [`AlwaysPassable`](super::wall::AlwaysPassable).
#[derive(Component)]
pub struct PassesThroughWalls;
