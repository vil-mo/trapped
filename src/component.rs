use bevy::prelude::Component;
use enumset::EnumSetType;

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

        let world = app.world_mut();
        world.register_component::<Group>();
    }
}

#[derive(Component, EnumSetType, Debug, Hash)]
pub enum Group {
    Red,
    Blue,
    Green,
    Yellow,
    Pink,
    Cyan,
}
