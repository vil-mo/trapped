use super::{
    components::{
        floor::{Floor, Unwalkable},
        object::{NeedsWalkableFloor, Object, PassesThroughWalls},
        wall::{AlwaysPassable, Wall},
    },
    spatial_index::SpatialIndex,
};
use crate::direction::Direction;
use bevy::ecs::{
    entity::Entity,
    query::{Has, With},
    system::{Query, Res, SystemState},
    world::World,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CanMove {
    Can,
    /// Entity that [`can_move`] is called on doesn't have [`Object`] component.
    NotAnObject,
    BumpedIntoWall(Entity),
    BumpedIntoObject(Entity),
    NoFloor,
    UnwalkableFloor(Entity),
}

impl CanMove {
    #[inline]
    pub fn to_bool(self) -> bool {
        match self {
            CanMove::Can => true,
            _ => false,
        }
    }
}

pub fn can_move(world: &mut World, entity: Entity, direction: Direction) -> CanMove {
    let mut system_state = SystemState::<(
        Res<SpatialIndex>,
        Query<(&Object, Has<NeedsWalkableFloor>, Has<PassesThroughWalls>)>,
        Query<Has<AlwaysPassable>, With<Wall>>,
        Query<Has<Unwalkable>, With<Floor>>,
    )>::new(world);
    let world: &World = world;

    let (spatial_index, object_query, wall_query, floor_query) = system_state.get(world);
    let Ok((object, needs_walkable_floor, passes_through_walls)) = object_query.get(entity) else {
        return CanMove::NotAnObject;
    };

    if !passes_through_walls {
        let wall = spatial_index.get_wall(object.pos, direction);
        if let Some(wall_entity) = wall {
            let always_passable = wall_query
                .get(wall_entity)
                .expect("`SpatialIndex` should return walls on wall request");

            if !always_passable {
                return CanMove::BumpedIntoWall(wall_entity);
            }
        }
    }

    let other_object = spatial_index.get_object(direction + object.pos);
    if let Some(other_object_entity) = other_object {
        return CanMove::BumpedIntoObject(other_object_entity);
    }

    if needs_walkable_floor {
        let Some(floor_entity) = spatial_index.get_floor(direction + object.pos) else {
            return CanMove::NoFloor;
        };

        let unwalkable = floor_query
            .get(floor_entity)
            .expect("`SpatialIndex` should return floor on floor request");

        if unwalkable {
            return CanMove::UnwalkableFloor(floor_entity);
        }
    }

    CanMove::Can
}
