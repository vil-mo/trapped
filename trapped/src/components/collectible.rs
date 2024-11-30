pub struct RegisterCollectibleComponentsPlugin;

impl bevy::app::Plugin for RegisterCollectibleComponentsPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        let world = app.world_mut();
        world.register_component::<Collectible>();
    }
}

pub use crate::positioning::Collectible;
