use std::mem;

use super::spatial_index::SpatialIndex;
use crate::direction::Direction;
use crate::{
    components::{
        floor::{Floor, Unwalkable},
        object::{NeedsWalkableFloor, Object, PassesThroughWalls},
        wall::{AlwaysPassable, Wall},
    },
    target::Target,
};
use bevy::prelude::Mut;
use bevy::{
    ecs::{
        entity::Entity,
        query::{Has, With},
        system::{Query, Res, SystemState},
        world::World,
    },
    math::IVec2,
};

#[derive(Clone, Copy)]
pub enum CanMoveEntity {
    Can,
    /// Entity that [`can_move`] is called on doesn't have [`Object`] component.
    NotAnObject,
    BumpedIntoWall(Entity),
    BumpedIntoObject(Entity),
    NoFloor,
    UnwalkableFloor(Entity),
}

impl CanMoveEntity {
    #[inline]
    pub fn to_bool(self) -> bool {
        matches!(self, CanMoveEntity::Can)
    }
}

pub fn can_move_entity(world: &World, entity: Entity, direction: Direction) -> CanMoveEntity {
    let (spatial_index, mut object_query, mut wall_query, mut floor_query) = (
        world.resource::<SpatialIndex>(),
        world
            .try_query::<(&Object, Has<NeedsWalkableFloor>, Has<PassesThroughWalls>)>()
            .unwrap(),
        world
            .try_query_filtered::<Has<AlwaysPassable>, With<Wall>>()
            .unwrap(),
        world
            .try_query_filtered::<Has<Unwalkable>, With<Floor>>()
            .unwrap(),
    );

    let Ok((object, needs_walkable_floor, passes_through_walls)) = object_query.get(world, entity)
    else {
        return CanMoveEntity::NotAnObject;
    };

    if !passes_through_walls {
        let wall = spatial_index.get_wall(object.pos, direction);
        if let Some(wall_entity) = wall {
            let always_passable = wall_query
                .get(world, wall_entity)
                .expect("`SpatialIndex` should return walls on wall request");

            if !always_passable {
                return CanMoveEntity::BumpedIntoWall(wall_entity);
            }
        }
    }

    let other_object = spatial_index.get_object(direction + object.pos);
    if let Some(other_object_entity) = other_object {
        return CanMoveEntity::BumpedIntoObject(other_object_entity);
    }

    if needs_walkable_floor {
        let Some(floor_entity) = spatial_index.get_floor(direction + object.pos) else {
            return CanMoveEntity::NoFloor;
        };

        let unwalkable = floor_query
            .get(world, floor_entity)
            .expect("`SpatialIndex` should return floor on floor request");

        if unwalkable {
            return CanMoveEntity::UnwalkableFloor(floor_entity);
        }
    }

    CanMoveEntity::Can
}

pub struct Bumped {
    pub initiator: Entity,
    pub into: Entity,
}

pub enum CanMove {
    /// Means there is no objects that are not a member of the target which are in front of each member of the target.
    /// Does not mean there is a free cell in front of the objects,
    /// so can't call [`translate`] on each individual object. Sholud call [`move_target`] instead.
    Can,
    /// Vec of entities that can't move because there is no floor
    NoFloor(Vec<Entity>),
    /// First Vec is entities that can't move because they bumped into unwalkable floor.
    /// Second Vec is entities that can't move because there is no floor.
    UnwalkableFloor(Vec<Bumped>, Vec<Entity>),
    /// Bumbed into, where `into` is not a member of the target.
    BumpedInto(Vec<Bumped>),
}

impl CanMove {
    fn add_no_floor(&mut self, entity: Entity) {
        match self {
            CanMove::Can => *self = CanMove::NoFloor(vec![entity]),
            CanMove::NoFloor(entities) => entities.push(entity),
            CanMove::UnwalkableFloor(_, entities) => entities.push(entity),
            CanMove::BumpedInto(_) => (),
        }
    }

    fn add_unwalkable_floor(&mut self, bumped: Bumped) {
        match self {
            CanMove::Can => *self = CanMove::UnwalkableFloor(vec![bumped], vec![]),
            CanMove::NoFloor(entities) => {
                let entities_vec = mem::take(entities);
                *self = CanMove::UnwalkableFloor(vec![bumped], entities_vec);
            }
            CanMove::UnwalkableFloor(bumpeds, _) => {
                bumpeds.push(bumped);
            }
            CanMove::BumpedInto(_) => (),
        }
    }

    fn add_bumped_into(&mut self, bumped: Bumped) {
        match self {
            CanMove::BumpedInto(bumpeds) => bumpeds.push(bumped),
            _ => *self = CanMove::BumpedInto(vec![bumped]),
        }
    }
}

pub fn can_move(world: &World, target: Target, direction: Direction) -> CanMove {
    let objects = target.fitting_objects(world);

    let mut can_move = CanMove::Can;

    for object in objects {
        let object_can_move = can_move_entity(world, object, direction);

        match object_can_move {
            CanMoveEntity::Can => continue,
            CanMoveEntity::BumpedIntoObject(entity) => {
                if !target.entity_matches(world, entity) {
                    can_move.add_bumped_into(Bumped {
                        initiator: object,
                        into: entity,
                    });
                }
            }
            CanMoveEntity::BumpedIntoWall(entity) => {
                can_move.add_bumped_into(Bumped {
                    initiator: object,
                    into: entity,
                });
            }
            CanMoveEntity::NoFloor => {
                can_move.add_no_floor(object);
            }
            CanMoveEntity::UnwalkableFloor(entity) => {
                can_move.add_unwalkable_floor(Bumped {
                    initiator: object,
                    into: entity,
                });
            }
            CanMoveEntity::NotAnObject => unreachable!(),
        }
    }

    can_move
}

/// CORRECTNESS: Should not be called if there is another object on the cell this object is trying to move to.
pub fn translate(world: &mut World, entity: Entity, direction: Direction) {
    world.resource_scope(|world, mut spatial_index: Mut<SpatialIndex>| {
        let Ok(mut entity_mut) = world.get_entity_mut(entity) else {
            return;
        };
        let Some(mut object) = entity_mut.get_mut::<Object>() else {
            return;
        };

        let removed = spatial_index.objects.remove(&*object);
        debug_assert_eq!(removed, Some(entity));

        object.pos = direction + object.pos;

        let replaced = spatial_index.objects.insert(*object, entity);
        debug_assert!(replaced.is_none());
    });
}

fn amount_of_neighbors_in_direction(
    pos: IVec2,
    direction: Direction,
    spatial_index: &SpatialIndex,
) -> i32 {
    let mut result = 0;
    let mut current_pos = direction + pos;
    while spatial_index.get_object(pos).is_some() {
        result += 1;
        current_pos = direction + current_pos;
    }
    result
}

fn has_neighbor_in_direction(
    pos: IVec2,
    direction: Direction,
    spatial_index: &SpatialIndex,
) -> bool {
    spatial_index.get_object(direction + pos).is_some()
}

/// CORRECTNESS: `can_move` with the same input arguments should not return `CanMove::BumpedInto`
pub fn move_target(world: &mut World, target: Target, direction: Direction) {
    let objects = target.fitting_objects(world);

    // n * log(n)
    // objects.sort_unstable_by_key(|&entity| {
    //     let pos = world.entity(entity).get::<Object>().unwrap().pos;
    //     amount_of_neighbors_in_direction(pos, direction, spatial_index)
    // });
    // for object in objects {
    //     translate(world, object, direction);
    // }

    // n * n worst case, but n is small + overwelmengly likely it will be way faster
    let mut objects = objects.into_iter().map(Some).collect::<Vec<_>>();
    let mut should_iterate = true;
    while should_iterate {
        should_iterate = false;
        for object_option in objects.iter_mut() {
            if let Some(object_entity) = object_option.as_mut().copied() {
                let entity = world.entity(object_entity);
                let object = entity.get::<Object>().unwrap();
                let spatial_index = world.resource::<SpatialIndex>();

                if has_neighbor_in_direction(object.pos, direction, spatial_index) {
                    should_iterate = true;
                } else {
                    translate(world, object_entity, direction);
                    *object_option = None;
                }
            }
        }
    }
}
