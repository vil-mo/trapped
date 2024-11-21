use bevy::prelude::Component;

pub use crate::positioning::Floor;

/// If object [`NeedsWalkableFloor`](super::object::NeedsWalkableFloor), it can't move onto this floor.
#[derive(Component)]
pub struct Unwalkable;
