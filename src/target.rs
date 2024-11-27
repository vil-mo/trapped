use crate::{
    actions::{Action, ActionEnum, ActionStatus},
    components::{collectible::Collectible, floor::Floor, object::Object, wall::Wall},
    direction::Direction,
    group::{Group, GroupComponent},
    positioning::spatial_index::SpatialIndex,
};
use bevy::{
    ecs::{entity::Entity, query::With, world::World},
    math::IVec2,
};

#[derive(Debug, Clone, Copy)]
pub enum Target {
    Group(Group),
    Cell(IVec2),
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
        pub fn $func_name(self, world: &mut World) -> Vec<Entity> {
            match self {
                Target::Group(group) => {
                    let mut query =
                        world.query_filtered::<(Entity, Option<&GroupComponent>), With<$t>>();

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
                spatial_index
                    .get_collectible(pos)
                    .map_or(false, |entity| entity == entity)
                    || spatial_index
                        .get_floor(pos)
                        .map_or(false, |entity| entity == entity)
                    || spatial_index
                        .get_object(pos)
                        .map_or(false, |entity| entity == entity)
                    || spatial_index
                        .get_wall(pos, Direction::Down)
                        .map_or(false, |entity| entity == entity)
                    || spatial_index
                        .get_wall(pos, Direction::Up)
                        .map_or(false, |entity| entity == entity)
                    || spatial_index
                        .get_wall(pos, Direction::Left)
                        .map_or(false, |entity| entity == entity)
                    || spatial_index
                        .get_wall(pos, Direction::Right)
                        .map_or(false, |entity| entity == entity)
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

#[derive(Clone, Copy)]
pub struct TargetedAction(pub ActionEnum, pub Target);

impl TargetedAction {
    pub fn apply(self, world: &mut World) -> ActionStatus {
        self.0.apply(self.1, world)
    }

    pub fn undo(self, world: &mut World) {
        self.0.undo(self.1, world)
    }
}
