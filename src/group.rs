use bevy::prelude::{Component, Entity};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Group {
    None(Entity),
    Red,
    Blue,
    Green,
    Yellow,
    Pink,
    Cyan,
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum GroupComponent {
    #[default]
    None,
    Red,
    Blue,
    Green,
    Yellow,
    Pink,
    Cyan,
}

impl Group {
    pub fn from_component(component: Option<GroupComponent>, entity: Entity) -> Self {
        match component {
            Some(GroupComponent::None) => Group::None(entity),
            Some(GroupComponent::Red) => Group::Red,
            Some(GroupComponent::Blue) => Group::Blue,
            Some(GroupComponent::Green) => Group::Green,
            Some(GroupComponent::Yellow) => Group::Yellow,
            Some(GroupComponent::Pink) => Group::Pink,
            Some(GroupComponent::Cyan) => Group::Cyan,
            None => Group::None(entity),
        }
    }

    pub fn to_component(self) -> GroupComponent {
        match self {
            Group::None(_) => GroupComponent::None,
            Group::Red => GroupComponent::Red,
            Group::Blue => GroupComponent::Blue,
            Group::Green => GroupComponent::Green,
            Group::Yellow => GroupComponent::Yellow,
            Group::Pink => GroupComponent::Pink,
            Group::Cyan => GroupComponent::Cyan,
        }
    }
}
