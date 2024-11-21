use bevy::{math::IVec2, prelude::Component};

pub mod can_move;
pub mod spatial_index;

pub struct PositioningPlugin;

impl bevy::app::Plugin for PositioningPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_plugins(spatial_index::SpatialIndexPlugin);
    }
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Collectible {
    pos: IVec2,
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Floor {
    pos: IVec2,
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Object {
    pos: IVec2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum WallAlignment {
    Up,
    Right,
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Wall {
    pos: IVec2,
    alignment: WallAlignment,
}
