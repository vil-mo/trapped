use crate::direction::Direction;
use glam::IVec2;
use plane::Plane;

#[allow(dead_code)]
mod plane;

pub struct Wall;
#[derive(Default)]
pub struct Walls {
    up: Option<Wall>,
    right: Option<Wall>,
}

pub struct Floor;
pub struct Collectible;
pub struct Object;


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Group {
    None,
    Red,
    Blue,
    Green,
    Yellow,
    Pink,
    Cyan,
}



macro_rules! impl_plane {
    ($name:ident, $inner:ty) => {
        #[derive(Default)]
        pub struct $name(Plane<Option<$inner>>);

        impl $name {
            #[inline]
            pub fn get(&self, pos: IVec2) -> Option<&$inner> {
                self.0.get(pos).as_ref()
            }

            #[inline]
            pub fn get_mut(&mut self, pos: IVec2) -> Option<&mut $inner> {
                self.0.get_mut(pos).as_mut()
            }

            #[inline]
            pub fn insert(&mut self, pos: IVec2, value: $inner) -> Option<$inner> {
                self.0.insert(pos, Some(value))
            }

            #[inline]
            pub fn remove(&mut self, pos: IVec2) -> Option<$inner> {
                self.0.inner_hash_map_mut().remove(&pos).flatten()
            }
        }
    };
}
impl_plane!(FloorPlane, Floor);
impl_plane!(CollectiblesPlane, Collectible);
impl_plane!(ObjectsPlane, Object);

pub struct WallsPlane(Plane<Walls>);

impl WallsPlane {
    pub fn get(&self, pos: IVec2, direction: Direction) -> Option<&Wall> {
        match direction {
            Direction::Up => self.0.get(pos).up.as_ref(),
            Direction::Down => self.0.get(direction + pos).up.as_ref(),
            Direction::Right => self.0.get(pos).right.as_ref(),
            Direction::Left => self.0.get(direction + pos).right.as_ref(),
        }
    }

    pub fn get_mut(&mut self, pos: IVec2, direction: Direction) -> Option<&mut Wall> {
        match direction {
            Direction::Up => self.0.get_mut(pos).up.as_mut(),
            Direction::Down => self.0.get_mut(direction + pos).up.as_mut(),
            Direction::Right => self.0.get_mut(pos).right.as_mut(),
            Direction::Left => self.0.get_mut(direction + pos).right.as_mut(),
        }
    }

    pub fn insert(&mut self, pos: IVec2, direction: Direction, value: Wall) -> Option<Wall> {
        match direction {
            Direction::Up => {
                let walls = self.0.get_mut(pos);
                walls.up.replace(value)
            }
            Direction::Down => {
                let walls = self.0.get_mut(direction + pos);
                walls.up.replace(value)
            }
            Direction::Right => {
                let walls = self.0.get_mut(pos);
                walls.right.replace(value)
            }
            Direction::Left => {
                let walls = self.0.get_mut(direction + pos);
                walls.right.replace(value)
            }
        }
    }

    pub fn remove(&mut self, pos: IVec2, direction: Direction) -> Option<Wall> {
        match direction {
            Direction::Up => {
                let walls = self.0.get_mut(pos);
                walls.up.take()
            }
            Direction::Down => {
                let walls = self.0.get_mut(direction + pos);
                walls.up.take()
            }
            Direction::Right => {
                let walls = self.0.get_mut(pos);
                walls.right.take()
            }
            Direction::Left => {
                let walls = self.0.get_mut(direction + pos);
                walls.right.take()
            }
        }
    }
}

pub struct Level {
    floor: FloorPlane,
    collectibles: CollectiblesPlane,
    objects: ObjectsPlane,
    walls: WallsPlane,

    min: IVec2,
    max: IVec2,
}
