use super::{StateChange, StateChangeEnum, Undo, UndoEnum};
use crate::level_state::{positioning::Positioning, LevelState};
use bevy::ecs::{entity::Entity, world::EntityWorldMut};

pub type SpawnDescription = fn(&mut EntityWorldMut);

pub struct Spawn(SpawnDescription, Positioning);
pub struct SpawnUndo(Entity);

impl StateChange for Spawn {
    type Undo = SpawnUndo;

    fn apply(self, level_state: &mut LevelState) -> Self::Undo {
        let entity = level_state.world.spawn_empty().id();
        (self.0)(&mut level_state.world.entity_mut(entity));
        SpawnUndo(entity)
    }
}

impl Undo<Spawn> for SpawnUndo {
    fn undo(self, level_state: &mut LevelState) {
        level_state.world.despawn(self.0);
    }
}

impl Into<StateChangeEnum> for Spawn {
    #[inline]
    fn into(self) -> StateChangeEnum {
        StateChangeEnum::Spawn(self)
    }
}

impl Into<UndoEnum> for SpawnUndo {
    #[inline]
    fn into(self) -> UndoEnum {
        UndoEnum::Spawn(self)
    }
}
