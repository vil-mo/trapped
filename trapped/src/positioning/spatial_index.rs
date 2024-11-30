use super::{Collectible, Floor, Object, Wall, WallAlignment};
use crate::direction::Direction;
use bevy::{
    app::{App, Plugin},
    ecs::{
        component::{ComponentHook, ComponentId},
        entity::Entity,
        system::Resource,
        world::{DeferredWorld, FromWorld, World},
    },
    math::IVec2,
    utils::HashMap,
};

pub struct SpatialIndexPlugin;

impl Plugin for SpatialIndexPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SpatialIndex>();
        let world = app.world_mut();
        world
            .register_component_hooks::<Collectible>()
            .on_add(on_add_collectible())
            .on_remove(on_remove_collectible());
        world
            .register_component_hooks::<Floor>()
            .on_add(on_add_floor())
            .on_remove(on_remove_floor());
        world
            .register_component_hooks::<Object>()
            .on_add(on_add_object())
            .on_remove(on_remove_object());
        world
            .register_component_hooks::<Wall>()
            .on_add(on_add_wall())
            .on_remove(on_remove_wall());
    }
}
macro_rules! on_add_si {
    ($t:ty, $hm:ident) => {
        |mut world: DeferredWorld, entity: Entity, _id: ComponentId| {
            let entity_ref = world.entity(entity);
            let component = *entity_ref.get::<$t>().unwrap();
            let mut spatial_index = world.resource_mut::<SpatialIndex>();
            spatial_index.$hm.insert(component, entity);
        }
    };
}

fn on_add_collectible() -> ComponentHook {
    on_add_si!(Collectible, collectibles)
}

fn on_add_floor() -> ComponentHook {
    on_add_si!(Floor, floor)
}

fn on_add_object() -> ComponentHook {
    on_add_si!(Object, objects)
}

fn on_add_wall() -> ComponentHook {
    on_add_si!(Wall, walls)
}

macro_rules! on_romove_si {
    ($t:ty, $hm:ident) => {
        |mut world: DeferredWorld, entity: Entity, _id: ComponentId| {
            let entity_ref = world.entity(entity);
            let component = *entity_ref.get::<$t>().unwrap();
            let mut spatial_index = world.resource_mut::<SpatialIndex>();
            spatial_index.$hm.remove(&component);
        }
    };
}

fn on_remove_collectible() -> ComponentHook {
    on_romove_si!(Collectible, collectibles)
}

fn on_remove_floor() -> ComponentHook {
    on_romove_si!(Floor, floor)
}

fn on_remove_object() -> ComponentHook {
    on_romove_si!(Object, objects)
}

fn on_remove_wall() -> ComponentHook {
    on_romove_si!(Wall, walls)
}

#[derive(Resource)]
pub struct SpatialIndex {
    pub(super) collectibles: HashMap<Collectible, Entity>,
    pub(super) floor: HashMap<Floor, Entity>,
    pub(super) objects: HashMap<Object, Entity>,
    pub(super) walls: HashMap<Wall, Entity>,
}

impl FromWorld for SpatialIndex {
    fn from_world(world: &mut World) -> Self {
        let mut collectibles = HashMap::new();
        let mut floor = HashMap::new();
        let mut objects = HashMap::new();
        let mut walls = HashMap::new();

        for (entity, &c) in world.query::<(Entity, &Collectible)>().iter(world) {
            collectibles.insert(c, entity);
        }

        for (entity, &f) in world.query::<(Entity, &Floor)>().iter(world) {
            floor.insert(f, entity);
        }

        for (entity, &o) in world.query::<(Entity, &Object)>().iter(world) {
            objects.insert(o, entity);
        }

        for (entity, &w) in world.query::<(Entity, &Wall)>().iter(world) {
            walls.insert(w, entity);
        }

        Self {
            collectibles,
            floor,
            objects,
            walls,
        }
    }
}

impl SpatialIndex {
    pub fn get_collectible(&self, pos: IVec2) -> Option<Entity> {
        self.collectibles.get(&Collectible { pos }).copied()
    }

    pub fn get_floor(&self, pos: IVec2) -> Option<Entity> {
        self.floor.get(&Floor { pos }).copied()
    }

    pub fn get_object(&self, pos: IVec2) -> Option<Entity> {
        self.objects.get(&Object { pos }).copied()
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
