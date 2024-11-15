use super::components::{
    collectible::Collectible,
    floor::Floor,
    object::Object,
    wall::{Wall, WallAlignment},
};
use crate::direction::Direction;
use bevy_ecs::{entity::Entity, system::Resource, world::FromWorld};
use bevy_math::IVec2;
use bevy_utils::HashMap;

#[derive(Resource)]
pub struct SpatialIndex {
    pub(super) collectibles: HashMap<Collectible, Entity>,
    pub(super) floor: HashMap<Floor, Entity>,
    pub(super) objects: HashMap<Object, Entity>,
    pub(super) walls: HashMap<Wall, Entity>,
}

impl FromWorld for SpatialIndex {
    fn from_world(world: &mut bevy_ecs::world::World) -> Self {
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
