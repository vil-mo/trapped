use super::{StateChange, StateChangeEnum, Undo, UndoEnum};
use crate::level_state::LevelState;
use bevy::math::IVec2;

pub struct Swap {
    pub pos1: IVec2,
    pub pos2: IVec2,
}

impl StateChange for Swap {
    type Undo = Self;

    fn apply(self, level_state: &mut LevelState) -> Self::Undo {
        let index = &mut level_state.root.spatial_index;
        index.swap_objects(self.pos1, self.pos2);
        self
    }
}

impl Undo<Swap> for Swap {
    fn undo(self, level_state: &mut LevelState) {
        self.apply(level_state);
    }
}

impl Into<StateChangeEnum> for Swap {
    #[inline]
    fn into(self) -> StateChangeEnum {
        StateChangeEnum::Swap(self)
    }
}

impl Into<UndoEnum> for Swap {
    #[inline]
    fn into(self) -> UndoEnum {
        UndoEnum::Swap(self)
    }
}
