use crate::positioning::spatial_index::SpatialIndex;
use enum_dispatch::enum_dispatch;

pub struct LevelState {
    spatial_index: SpatialIndex,
}

#[enum_dispatch]
pub trait StateChange {
    fn apply(self, level_state: &mut LevelState);

    fn undo(self, level_state: &mut LevelState);
}

#[enum_dispatch(StateChange)]
pub enum StateChangeEnum {}
