use glam::IVec2;
use std::ops::Add;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Into<IVec2> for Direction {
    fn into(self) -> IVec2 {
        match self {
            Direction::Up => IVec2::Y,
            Direction::Down => IVec2::NEG_Y,
            Direction::Left => IVec2::X,
            Direction::Right => IVec2::NEG_X,
        }
    }
}

impl Add<IVec2> for Direction {
    type Output = IVec2;

    fn add(self, rhs: IVec2) -> Self::Output {
        rhs + Into::<IVec2>::into(self)
    }
}
