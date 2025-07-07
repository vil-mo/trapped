pub struct RegisterCollectibleComponentsPlugin;

impl bevy::app::Plugin for RegisterCollectibleComponentsPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        let world = app.world_mut();
        world.register_component::<Collectible>();
    }
}

pub use crate::level_state::positioning::Collectible;
use crate::{action::ActionResult, level_state::{CollectibleId, LevelState}};

pub struct OnActivated {
    callback: fn(CollectibleId, &mut LevelState) -> ActionResult,
}

