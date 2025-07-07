use super::LevelState;

pub mod destroy;
pub mod spawn;
pub mod swap;

pub trait StateChange: Into<StateChangeEnum> {
    type Undo: Undo<Self>;

    fn apply(self, level_state: &mut LevelState) -> Self::Undo;
}

pub trait Undo<T: StateChange>: Into<UndoEnum> {
    fn undo(self, level_state: &mut LevelState);
}

pub enum StateChangeEnum {
    Destroy(destroy::Destroy),
    Spawn(spawn::Spawn),
    Swap(swap::Swap),
}

impl StateChangeEnum {
    pub fn apply(self, level_state: &mut LevelState) -> UndoEnum {
        match self {
            StateChangeEnum::Destroy(destroy) => destroy.apply(level_state).into(),
            StateChangeEnum::Spawn(spawn) => spawn.apply(level_state).into(),
            StateChangeEnum::Swap(swap) => swap.apply(level_state).into(),
        }
    }
}

pub enum UndoEnum {
    NextBatch,
    Destroy(<destroy::Destroy as StateChange>::Undo),
    Spawn(<spawn::Spawn as StateChange>::Undo),
    Swap(<swap::Swap as StateChange>::Undo),
}

impl UndoEnum {
    pub fn undo(self, level_state: &mut LevelState) {
        match self {
            UndoEnum::NextBatch => (),
            UndoEnum::Destroy(destroy) => destroy.undo(level_state),
            UndoEnum::Spawn(spawn) => spawn.undo(level_state),
            UndoEnum::Swap(swap) => swap.undo(level_state),
        }
    }
}
