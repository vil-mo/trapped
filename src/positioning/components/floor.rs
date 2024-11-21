use bevy::{ecs::component::Component, math::IVec2};

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Floor {
    pub(in super::super) pos: IVec2,
}

/// If object [`NeedsWalkableFloor`](super::object::NeedsWalkableFloor), it can't move onto this floor.
#[derive(Component)]
pub struct Unwalkable;
