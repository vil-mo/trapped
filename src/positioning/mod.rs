pub mod action;
pub mod can_move;
pub mod components;
pub mod spatial_index;

pub struct PositioningPlugin;

impl bevy::app::Plugin for PositioningPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_plugins(spatial_index::SpatialIndexPlugin);
    }
}
