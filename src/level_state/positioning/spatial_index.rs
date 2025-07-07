use super::{Collectible, Floor, Object, Positioning, Wall, WallAlignment};
use crate::{direction::Direction, level_state::{CollectibleId, FloorId, ObjectId, WallId}};
use bevy::{ecs::entity::{self, Entity}, math::IVec2, platform_support::collections::HashMap};
pub struct SpatialIndex {
    collectibles: HashMap<Collectible, CollectibleId>,
    floor: HashMap<Floor, FloorId>,
    objects: HashMap<Object, ObjectId>,
    walls: HashMap<Wall, WallId>,
}

impl SpatialIndex {
    pub fn get_collectible(&self, pos: IVec2) -> Option<CollectibleId> {
        self.collectibles.get(&Collectible { pos }).copied()
    }

    pub fn get_floor(&self, pos: IVec2) -> Option<FloorId> {
        self.floor.get(&Floor { pos }).copied()
    }

    pub fn get_object(&self, pos: IVec2) -> Option<ObjectId> {
        self.objects.get(&Object { pos }).copied()
    }

    #[rustfmt::skip]
    pub fn get_wall(&self, pos: IVec2, direction: Direction) -> Option<WallId> {
        self.walls.get(&match direction {
            Direction::Down  => Wall { pos: direction + pos, alignment: WallAlignment::Up },
            Direction::Up    => Wall { pos,                  alignment: WallAlignment::Up },
            Direction::Left  => Wall { pos: direction + pos, alignment: WallAlignment::Right },
            Direction::Right => Wall { pos,                  alignment: WallAlignment::Right },
        }).copied()
    }

    pub fn swap_objects(&mut self, pos1: IVec2, pos2: IVec2) {
        let object1 = Object { pos: pos1 };
        let object2 = Object { pos: pos2 };

        let objects = self.objects.get_many_mut([&object1, &object2]);
        match objects {
            [Some(o1), Some(o2)] => {
                std::mem::swap(o1, o2);
            }
            [Some(_), None] => {
                let o1 = self.objects.remove(&object1);
                self.objects.insert(object2, o1.unwrap());
            }
            [None, Some(_)] => {
                let o2 = self.objects.remove(&object2);
                self.objects.insert(object1, o2.unwrap());
            }
            _ => (),
        }
    }

    pub fn spawn(&mut self, positioning: Positioning, entity: Entity) {
        match positioning {
            Positioning::Collectible(collectible) => {
                self.collectibles.insert(collectible, CollectibleId(entity));
            }
            Positioning::Floor(floor) => {
                self.floor.insert(floor, FloorId(entity));
            }
            Positioning::Object(object) => {
                self.objects.insert(object, ObjectId(entity));
            }
            Positioning::Wall(wall) => {
                self.walls.insert(wall, WallId(entity));
            }
        }
    }

    pub fn despawn(&mut self, positioning: Positioning) {
        match positioning {
            Positioning::Collectible(collectible) => {
                self.collectibles.remove(&collectible);
            }
            Positioning::Floor(floor) => {
                self.floor.remove(&floor);
            }
            Positioning::Object(object) => {
                self.objects.remove(&object);
            }
            Positioning::Wall(wall) => {
                self.walls.remove(&wall);
            }
        }
    }
}
