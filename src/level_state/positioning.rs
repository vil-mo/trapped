use bevy::{ecs::{component::Component, world::EntityWorldMut}, math::IVec2};

// pub mod movement;
pub mod spatial_index;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Collectible {
    pos: IVec2,
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Floor {
    pos: IVec2,
}

// TODO
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Object {
    pub(crate) pos: IVec2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum WallAlignment {
    Up,
    Right,
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Wall {
    pos: IVec2,
    alignment: WallAlignment,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Positioning {
    Collectible(Collectible),
    Floor(Floor),
    Object(Object),
    Wall(Wall),
}

impl Positioning {
    pub fn insert(self, mut entity: EntityWorldMut) {
        match self {
            Positioning::Collectible(collectible) => {
                entity.insert(collectible);
            }
            Positioning::Floor(floor) => {
                entity.insert(floor);
            }
            Positioning::Object(object) => {
                entity.insert(object);
            }
            Positioning::Wall(wall) => {
                entity.insert(wall);
            }
        }
    }

    pub fn remove(mut entity: EntityWorldMut) -> Self {
    }
}
