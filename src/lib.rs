use std::collections::VecDeque;

use bevy_ecs::{
    component::Component,
    entity::{self, Entity},
    event::{Event, EventCursor, EventReader, Events},
    query::{QueryData, QueryEntityError, QueryFilter, QueryItem, QueryState},
    system::{BoxedSystem, In, IntoSystem, Local, Resource, System},
    world::{unsafe_world_cell::UnsafeWorldCell, FromWorld, Mut, World},
};
use bevy_math::IVec2;
use bevy_utils::HashMap;
use direction::Direction;
use enum_dispatch::enum_dispatch;

mod direction;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Event)]
pub enum Signal {
    Move(Direction),
    Step,
}

pub struct WorldContext<'a> {
    pub world: &'a mut World,
    pub spatial_index: &'a mut SpatialIndex,
}

impl<'a> WorldContext<'a> {
    pub fn reborrow<'b>(&'b mut self) -> WorldContext<'b> {
        WorldContext {
            world: self.world,
            spatial_index: self.spatial_index,
        }
    }

    pub fn can_move(&self, from: IVec2, direction: Direction) -> bool {
        todo!();
        false
    }
}

pub type Actions = VecDeque<ActionEnum>;

#[enum_dispatch]
pub trait Action: Copy {
    fn apply(self, target: Target, context: WorldContext, actions: &mut Actions);
    fn undo(self, target: Target, context: WorldContext);
}

#[derive(Clone, Copy)]
pub struct MoveWithoutPush(pub Direction);

impl Action for MoveWithoutPush {
    fn apply(self, target: Target, mut context: WorldContext, _actions: &mut Actions) {
        let mut query = context.world.query::<&mut Object>();
        for entity in target.iter(context.reborrow()) {
            let Ok(object) = query.get(context.world, entity) else {
                continue;
            };
            let pos = object.pos;
            if !context.can_move(pos, self.0) {
                continue;
            }
            let Ok(mut object) = query.get_mut(context.world, entity) else {
                continue;
            };

            let removed = context.spatial_index.objects.remove(&*object);
            debug_assert!(removed.is_some());
            object.pos = self.0 + object.pos;
            let replaced = context.spatial_index.objects.insert(*object, entity);
            debug_assert!(replaced.is_none());
        }
    }

    fn undo(self, target: Target, mut context: WorldContext) {
        let mut query = context.world.query::<&mut Object>();
        for entity in target.iter(context.reborrow()) {
            let Ok(mut object) = query.get_mut(context.world, entity) else {
                continue;
            };

            let removed = context.spatial_index.objects.remove(&*object);
            debug_assert!(removed.is_some());
            object.pos = !self.0 + object.pos;
            let replaced = context.spatial_index.objects.insert(*object, entity);
            debug_assert!(replaced.is_none());
        }
    }
}

#[derive(Clone, Copy)]
pub struct MoveWithPush(pub Direction);

#[derive(Clone, Copy)]
pub struct Push(pub Direction);

#[enum_dispatch(Action)]
#[derive(Clone, Copy)]
pub enum ActionEnum {
    //MoveWithoutPush,
}

#[derive(Debug, Clone, Copy)]
pub enum Cell {
    Of(Entity),
    Neighbour(Entity, Direction),
    Coord(IVec2),
}

#[derive(Debug, Clone, Copy)]
pub enum Target {
    Group(Group),
    Cell(Cell),
}

impl Target {
    pub fn iter(self, context: WorldContext) -> impl Iterator<Item = Entity> {
        todo!();
        std::iter::empty()
    }
}

pub enum TakeAction {
    None,
    Take(ActionEnum, Target),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Group {
    None(Entity),
    Red,
    Blue,
    Green,
    Yellow,
    Pink,
    Cyan,
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GroupComponent {
    None,
    Red,
    Blue,
    Green,
    Yellow,
    Pink,
    Cyan,
}

impl Group {
    pub fn from_component(component: Option<GroupComponent>, entity: Entity) -> Self {
        match component {
            Some(GroupComponent::None) => Group::None(entity),
            Some(GroupComponent::Red) => Group::Red,
            Some(GroupComponent::Blue) => Group::Blue,
            Some(GroupComponent::Green) => Group::Green,
            Some(GroupComponent::Yellow) => Group::Yellow,
            Some(GroupComponent::Pink) => Group::Pink,
            Some(GroupComponent::Cyan) => Group::Cyan,
            None => Group::None(entity),
        }
    }
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Floor {
    pos: IVec2,
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Object {
    pos: IVec2,
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Collectible {
    pos: IVec2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WallAlignment {
    Up,
    Right,
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Wall {
    pos: IVec2,
    alignment: WallAlignment,
}

pub struct SpatialIndex {
    floor: HashMap<Floor, Entity>,
    objects: HashMap<Object, Entity>,
    collectibles: HashMap<Collectible, Entity>,
    walls: HashMap<Wall, Entity>,
}

impl FromWorld for SpatialIndex {
    fn from_world(world: &mut bevy_ecs::world::World) -> Self {
        let mut floor = HashMap::new();
        let mut objects = HashMap::new();
        let mut collectibles = HashMap::new();
        let mut walls = HashMap::new();

        for (entity, &f) in world.query::<(Entity, &Floor)>().iter(world) {
            floor.insert(f, entity);
        }

        for (entity, &o) in world.query::<(Entity, &Object)>().iter(world) {
            objects.insert(o, entity);
        }

        for (entity, &c) in world.query::<(Entity, &Collectible)>().iter(world) {
            collectibles.insert(c, entity);
        }

        for (entity, &w) in world.query::<(Entity, &Wall)>().iter(world) {
            walls.insert(w, entity);
        }

        Self {
            floor,
            objects,
            collectibles,
            walls,
        }
    }
}

impl SpatialIndex {
    pub fn get_floor(&self, pos: IVec2) -> Option<Entity> {
        self.floor.get(&Floor { pos }).copied()
    }

    pub fn get_object(&self, pos: IVec2) -> Option<Entity> {
        self.objects.get(&Object { pos }).copied()
    }

    pub fn get_collectible(&self, pos: IVec2) -> Option<Entity> {
        self.collectibles.get(&Collectible { pos }).copied()
    }

    #[rustfmt::skip]
    pub fn get_wall(&self, pos: IVec2, direction: Direction) -> Option<Entity> {
        self.walls.get(&match direction {
            Direction::Down  => Wall { pos: direction + pos, alignment: WallAlignment::Up },
            Direction::Up    => Wall { pos,                  alignment: WallAlignment::Up },
            Direction::Left  => Wall { pos: direction + pos, alignment: WallAlignment::Right },
            Direction::Right => Wall { pos,                  alignment: WallAlignment::Right },
        }).copied()
    }
}

#[derive(Resource)]
pub struct ProcessSingnalsPasses {
    signal_reactions: Vec<BoxedSystem<In<Signal>, TakeAction>>,
}

fn process_signals(world: &mut World, mut signal_cursor: Local<EventCursor<Signal>>) {
    let Some(mut passes) = world.remove_resource::<ProcessSingnalsPasses>() else {
        return;
    };

    let mut signals = VecDeque::from_iter(
        signal_cursor
            .read(world.resource::<Events<Signal>>())
            .copied(),
    );

    while let Some(signal) = signals.pop_front() {
        for pass in &mut passes.signal_reactions {
            let take_action = pass.run(signal, world);
        }
    }

    world.insert_resource(passes);
}
