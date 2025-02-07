use bevy::prelude::{Component, Entity};
use enumset::EnumSetType;

#[derive(Component, EnumSetType, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Group {
    #[default]
    None,
    Red,
    Blue,
    Green,
    Yellow,
    Pink,
    Cyan,
}
