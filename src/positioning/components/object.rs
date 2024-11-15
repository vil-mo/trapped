use bevy_ecs::component::Component;
use bevy_math::IVec2;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Object {
    pub(in super::super) pos: IVec2,
}

/// Object only can move if in the position it moves onto there is a
/// [`Floor`](super::floor::Floor) that is not [`Unwalkable`](super::floor::Unwalkable).
#[derive(Component)]
pub struct NeedsWalkableFloor;

/// Object can pass through walls even if it's not [`AlwaysPassable`](super::wall::AlwaysPassable).
#[derive(Component)]
pub struct PassesThroughWalls;
