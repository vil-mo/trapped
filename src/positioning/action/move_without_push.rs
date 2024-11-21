use crate::{
    direction::Direction,
    positioning::{
        can_move::can_move, components::object::Object, spatial_index::SpatialIndex, target::Target,
    }, MakeStep,
};
use bevy::ecs::{prelude::*, system::SystemState};

use super::{Action, Actions};

#[derive(Clone, Copy)]
pub struct MoveWithoutPush(pub Direction);

impl Action for MoveWithoutPush {
    fn is_being_applied(&mut self, world: &mut World) -> bool {
        false
    }

    fn begin_apply(&mut self, target: Target, world: &mut World, _actions: &mut Actions) -> MakeStep {
        let mut system_state =
            SystemState::<(Query<&mut Object>, ResMut<SpatialIndex>)>::new(world);

        let mut make_step = MakeStep::DontMake;

        //TODO: fix
        for entity in target.fitting_objects(world) {
            if !can_move(world, entity, self.0).to_bool() {
                continue;
            }
            make_step = make_step | MakeStep::Make;

            let (mut query, mut spatial_index) = system_state.get_mut(world);

            let Ok(mut object) = query.get_mut(entity) else {
                continue;
            };

            let removed = spatial_index.objects.remove(&*object);
            debug_assert_eq!(removed, Some(entity));
            object.pos = self.0 + object.pos;
            let replaced = spatial_index.objects.insert(*object, entity);
            debug_assert!(replaced.is_none());
        }

        make_step
    }

    fn undo(&mut self, target: Target, world: &mut World) {
        let mut system_state =
            SystemState::<(Query<&mut Object>, ResMut<SpatialIndex>)>::new(world);

        for entity in target.fitting_objects(world) {
            let (mut query, mut spatial_index) = system_state.get_mut(world);
            let Ok(mut object) = query.get_mut(entity) else {
                continue;
            };

            let removed = spatial_index.objects.remove(&*object);
            debug_assert_eq!(removed, Some(entity));
            object.pos = !self.0 + object.pos;
            let replaced = spatial_index.objects.insert(*object, entity);
            debug_assert!(replaced.is_none());
        }
    }
}
