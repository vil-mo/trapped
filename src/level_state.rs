use bevy::ecs::{component::Component, entity::Entity, world::World};
use positioning::spatial_index::SpatialIndex;
use state_change::{StateChangeEnum, UndoEnum};

pub mod positioning;
pub mod state_change;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct CollectibleId(Entity);

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct FloorId(Entity);

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ObjectId(Entity);

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct WallId(Entity);

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum ItemId {
    Collectible(CollectibleId),
    Floor(FloorId),
    Object(ObjectId),
    Wall(WallId),
}

impl ItemId {
    pub fn entity(&self) -> Entity {
        match self {
            ItemId::Collectible(id) => id.0,
            ItemId::Floor(id) => id.0,
            ItemId::Object(id) => id.0,
            ItemId::Wall(id) => id.0,
        }
    }
}

#[derive(Component)]
pub struct LevelRoot {
    spatial_index: SpatialIndex,
    undo_stack: Vec<UndoEnum>,
}

pub struct LevelState<'w> {
    world: &'w mut World,
    root: LevelRoot,
}

impl<'w> LevelState<'w> {
    pub fn state_change(&mut self, state_change: StateChangeEnum) {
        let undo = state_change.apply(self);
        self.root.undo_stack.push(undo);
    }

    pub fn undo(&mut self) {
        if let Some(undo) = self.root.undo_stack.pop() {
            undo.undo(self);
        }
    }
}
