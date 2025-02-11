use crate::{
    actions::{Action, ActionEnum, ActionResult},
    components::{collectible::Collectible, floor::Floor, object::Object, wall::Wall},
    direction::Direction,
    group::{Group, GroupComponent},
    positioning::spatial_index::SpatialIndex,
};
use bevy::{
    ecs::{entity::Entity, query::With, world::World},
    math::IVec2,
};
use enumset::{EnumSet, EnumSetType};

pub trait TargetFilter {
    fn matches(&self, level_state: &LevelState) -> bool;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TargetType {
    Collectible,
    Floor,
    Object,
    Wall,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ExactTarget {
    Collectible(IVec2),
    Floor(IVec2),
    Object(IVec2),
    Wall(IVec2, Direction),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Target {
    /// In addition to matching everything else this target matches, it will also match those targets.
    pub cells: Vec<ExactTarget>,
    /// Targets all and only entities in the group that belongs to the set.
    pub group: EnumSet<Group>,
}

enum EitherIterator<A, B> {
    A(A),
    B(B),
}

impl<A: Iterator<Item = Entity>, B: Iterator<Item = Entity>> Iterator for EitherIterator<A, B> {
    type Item = Entity;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            EitherIterator::A(a) => a.next(),
            EitherIterator::B(b) => b.next(),
        }
    }
}

macro_rules! fitting_target {
    ($func_name:ident, $t:ty, $cell:expr) => {
        pub fn $func_name(self, world: &World) -> Vec<Entity> {
            match self {
                Target::Group(group) => {
                    let mut query = world
                        .try_query_filtered::<(Entity, Option<&GroupComponent>), With<$t>>()
                        .unwrap();

                    query
                        .iter(world)
                        .filter_map(|(entity, group_component)| {
                            if Group::from_component(group_component.copied(), entity) == group {
                                Some(entity)
                            } else {
                                None
                            }
                        })
                        .collect()
                }
                Target::Cell(pos) => {
                    let spatial_index = world.resource::<SpatialIndex>();
                    ($cell)(spatial_index, pos).into_iter().collect()
                }
            }
        }
    };
}

macro_rules! map_or_entitiy_eq {
    ($entity:expr, $($fns:expr,)*) => {
        false $(|| ($fns)().map_or(false, |e| e == $entity))*
    };
}

impl Target {
    /// True if the entity matches the target.
    pub fn entity_matches(&self, world: &World, entity: Entity) -> bool {
        match *self {
            Target::Group(group) => world
                .entity(entity)
                .get::<GroupComponent>()
                .map_or(false, |&group_component| {
                    group_component == group.to_component()
                }),
            Target::Cell(pos) => {
                let spatial_index = world.resource::<SpatialIndex>();
                map_or_entitiy_eq!(entity, 
                    || spatial_index.get_collectible(pos),
                    || spatial_index.get_floor(pos),
                    || spatial_index.get_object(pos),
                    || spatial_index.get_wall(pos, Direction::Down),
                    || spatial_index.get_wall(pos, Direction::Up),
                    || spatial_index.get_wall(pos, Direction::Left),
                    || spatial_index.get_wall(pos, Direction::Right),
                )
            }
        }
    }

    fitting_target!(
        fitting_collectibles,
        Collectible,
        |spatial_index: &SpatialIndex, pos| { spatial_index.get_collectible(pos).into_iter() }
    );
    fitting_target!(
        fitting_floors,
        Floor,
        |spatial_index: &SpatialIndex, pos| { spatial_index.get_floor(pos).into_iter() }
    );
    fitting_target!(
        fitting_objects,
        Object,
        |spatial_index: &SpatialIndex, pos| { spatial_index.get_object(pos).into_iter() }
    );
    fitting_target!(fitting_walls, Wall, |spatial_index: &SpatialIndex, pos| {
        let directions = [
            Direction::Left,
            Direction::Right,
            Direction::Up,
            Direction::Down,
        ]
        .map(|direction| spatial_index.get_wall(pos, direction).into_iter());

        directions.into_iter().flatten()
    });
}
