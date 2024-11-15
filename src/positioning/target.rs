use super::{
    components::{collectible::Collectible, floor::Floor, object::Object, wall::Wall},
    spatial_index::SpatialIndex,
};
use crate::{direction::Direction, Group, GroupComponent};
use bevy_ecs::{entity::Entity, query::With, world::World};
use bevy_math::IVec2;

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
        pub fn $func_name(self, world: &mut World) -> impl Iterator<Item = Entity> {
            match self {
                Target::Group(group) => {
                    let mut query =
                        world.query_filtered::<(Entity, Option<&GroupComponent>), With<$t>>();

                    let v = query
                        .iter(world)
                        .filter_map(|(entity, group_component)| {
                            if Group::from_component(group_component.copied(), entity) == group {
                                Some(entity)
                            } else {
                                None
                            }
                        })
                        .collect::<Vec<_>>();
                    EitherIterator::A(v.into_iter())
                }
                Target::Cell(pos) => {
                    let spatial_index = world.resource::<SpatialIndex>();
                    EitherIterator::B(($cell)(spatial_index, pos))
                }
            }
        }
    };
}

impl Target {
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
