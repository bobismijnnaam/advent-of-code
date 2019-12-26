use nalgebra::Vector2;
use std::convert::TryFrom;
use std::fmt::{Display, Formatter, Error};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Direction {
    North,
    East,
    South,
    West
}

impl Direction {

    pub fn right(&self) -> Direction {
        use Direction::*;
        match self {
            North => East,
            East => South,
            South => West,
            West => North
        }
    }

    pub fn left(&self) -> Direction {
        self.right().right().right()
    }

    pub fn mirror(&self) -> Direction {
        use Direction::*;
        match self {
            North => South,
            South => North,
            East => West,
            West => East,
        }
    }

    pub fn to_vec(&self) -> Vector2<i32> {
        use Direction::*;
        match self {
            North => Vector2::new(0, -1),
            East => Vector2::new(1, 0),
            South => Vector2::new(0, 1),
            West => Vector2::new(-1, 0)
        }
    }

    pub fn to_string(&self) -> &str {
        use Direction::*;
        match self {
            North => "north",
            East => "east",
            South => "south",
            West => "west",
        }
    }
}

impl From<Direction> for i32 {
    fn from(dir: Direction) -> i32 {
        use Direction::*;
        match dir {
            North => 1,
            South => 2,
            West => 3,
            East => 4,
        }
    }
}

impl TryFrom<&str> for Direction {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        use Direction::*;
        match value {
            "north" => Ok(North),
            "east" => Ok(East),
            "south" => Ok(South),
            "west" => Ok(West),
            _ => Err(()),
        }
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{}", self.to_string())
    }
}