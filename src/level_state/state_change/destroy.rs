use super::{StateChange, StateChangeEnum, Undo, UndoEnum};
use crate::level_state::{ItemId, LevelState};
use bevy::ecs::{entity_disabling::Disabled, hierarchy::Children};

pub struct Destroy(ItemId);

impl StateChange for Destroy {
    type Undo = Self;
    fn apply(self, level_state: &mut LevelState) -> Self::Undo {
        let entity = self.0.entity();

        level_state
            .world
            .entity_mut(entity)
            .insert_recursive::<Children>(Disabled);

        self
    }
}
impl Undo<Destroy> for Destroy {
    fn undo(self, level_state: &mut LevelState) {
        let entity = self.0.entity();

        level_state
            .world
            .entity_mut(entity)
            .remove_recursive::<Children, Disabled>();
    }
}

impl Into<StateChangeEnum> for Destroy {
    #[inline]
    fn into(self) -> StateChangeEnum {
        StateChangeEnum::Destroy(self)
    }
}

impl Into<UndoEnum> for Destroy {
    #[inline]
    fn into(self) -> UndoEnum {
        UndoEnum::Destroy(self)
    }
}
