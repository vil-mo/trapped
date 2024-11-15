use bevy_ecs::component::Component;
use bevy_math::IVec2;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Collectible {
    pub(in super::super) pos: IVec2,
}
