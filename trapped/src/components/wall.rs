use bevy::prelude::Component;

pub struct RegisterWallComponentsPlugin;

impl bevy::app::Plugin for RegisterWallComponentsPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        let world = app.world_mut();
        world.register_component::<Wall>();
        world.register_component::<AlwaysPassable>();
    }
}

pub use crate::positioning::Wall;

/// Object can pass through this wall even if it's not [`PassesThroughWalls`](super::object::PassesThroughWalls).
#[derive(Component)]
pub struct AlwaysPassable;
