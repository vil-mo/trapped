use bevy::prelude::Component;

pub struct RegisterFloorComponentsPlugin;

impl bevy::app::Plugin for RegisterFloorComponentsPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        let world = app.world_mut();
        world.register_component::<Floor>();
        world.register_component::<Unwalkable>();
    }
}

pub use crate::positioning::Floor;

/// If object [`NeedsWalkableFloor`](super::object::NeedsWalkableFloor), it can't move onto this floor.
#[derive(Component)]
pub struct Unwalkable;
