use bevy::prelude::Component;

pub struct RegisterWallComponentsPlugin;

impl bevy::app::Plugin for RegisterWallComponentsPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        let world = app.world_mut();
        world.register_component::<Wall>();
    }
}

pub use crate::level_state::positioning::Wall;
use crate::{action::ActionResult, level_state::{LevelState, WallId}};

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Opened(pub bool);

impl Default for Opened {
    #[inline]
    fn default() -> Self {
        Self(false)
    }
}

pub struct OnActivated {
    callback: fn(WallId, &mut LevelState) -> ActionResult,
}
