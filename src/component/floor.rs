pub struct RegisterFloorComponentsPlugin;

impl bevy::app::Plugin for RegisterFloorComponentsPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        let world = app.world_mut();
        world.register_component::<Floor>();
    }
}

pub use crate::level_state::positioning::Floor;
use crate::{action::ActionResult, level_state::{FloorId, LevelState}};

pub struct OnActivated {
    callback: fn(FloorId, &mut LevelState) -> ActionResult,
}

