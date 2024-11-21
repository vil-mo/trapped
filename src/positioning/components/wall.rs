use bevy::{ecs::component::Component, math::IVec2};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WallAlignment {
    Up,
    Right,
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Wall {
    pub(in super::super) pos: IVec2,
    pub(in super::super) alignment: WallAlignment,
}

/// Object can pass through this wall even if it's not [`PassesThroughWalls`](super::object::PassesThroughWalls).
#[derive(Component)]
pub struct AlwaysPassable;
