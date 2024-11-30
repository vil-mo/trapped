pub mod collectible;
pub mod floor;
pub mod object;
pub mod wall;

pub struct RegisterComponentsPlugin;

impl bevy::app::Plugin for RegisterComponentsPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_plugins((
            collectible::RegisterCollectibleComponentsPlugin,
            floor::RegisterFloorComponentsPlugin,
            object::RegisterObjectComponentsPlugin,
            wall::RegisterWallComponentsPlugin,
        ));
    }
}