use bevy::math::IVec2;
use std::ops::{Add, Not};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    Down,
    Up,
    Left,
    Right,
}

impl Into<IVec2> for Direction {
    fn into(self) -> IVec2 {
        match self {
            Direction::Down => IVec2::NEG_Y,
            Direction::Up => IVec2::Y,
            Direction::Left => IVec2::NEG_X,
            Direction::Right => IVec2::X,
        }
    }
}

impl Not for Direction {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Direction::Down => Direction::Up,
            Direction::Up => Direction::Down,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

impl Add<IVec2> for Direction {
    type Output = IVec2;

    fn add(self, rhs: IVec2) -> Self::Output {
        rhs + Into::<IVec2>::into(self)
    }
}
