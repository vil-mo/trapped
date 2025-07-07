use bevy::prelude::Component;

pub struct RegisterObjectComponentsPlugin;

impl bevy::app::Plugin for RegisterObjectComponentsPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        let world = app.world_mut();
        world.register_component::<Object>();
    }
}

pub use crate::level_state::positioning::Object;
use crate::{action::ActionResult, level_state::{LevelState, ObjectId}};

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NeedsWalkableFloor(pub bool);

impl Default for NeedsWalkableFloor {
    #[inline]
    fn default() -> Self {
        Self(true)
    }
}

pub struct OnActivated {
    callback: fn(ObjectId, &mut LevelState) -> ActionResult,
}
