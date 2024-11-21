use bevy::prelude::Component;

pub use crate::positioning::Wall;

/// Object can pass through this wall even if it's not [`PassesThroughWalls`](super::object::PassesThroughWalls).
#[derive(Component)]
pub struct AlwaysPassable;
