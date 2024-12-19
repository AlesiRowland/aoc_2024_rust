use std::fs::OpenOptions;
use std::mem;
use std::num::{ParseIntError, TryFromIntError};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Point<T: Copy> {
    pub x: T,
    pub y: T,
}


pub type Coordinate = Point<isize>;
pub type Index = Point<usize>;

impl TryFrom<Coordinate> for Index {
    type Error = TryFromIntError;

    fn try_from(value: Coordinate) -> Result<Index, TryFromIntError> {
        Ok(Index {
            x: value.x.try_into()?,
            y: value.y.try_into()?,
        })
    }
}
impl TryFrom<&Coordinate> for Index {
    type Error = TryFromIntError;

    fn try_from(value: &Coordinate) -> Result<Index, TryFromIntError> {
        Ok(Index {
            x: value.x.try_into()?,
            y: value.y.try_into()?,
        })
    }
}
impl TryFrom<Index> for Coordinate {
    type Error = TryFromIntError;

    fn try_from(value: Index) -> Result<Coordinate, TryFromIntError> {
        Ok(Coordinate {
            x: value.x.try_into()?,
            y: value.y.try_into()?,
        })
    }
}
impl TryFrom<&Index> for Coordinate {
    type Error = TryFromIntError;

    fn try_from(value: &Index) -> Result<Coordinate, TryFromIntError> {
        Ok(Coordinate {
            x: value.x.try_into()?,
            y: value.y.try_into()?,
        })
    }
}

impl<T: Copy> Point<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl Coordinate {
    pub(crate) fn shift(&self, direction: &Direction) -> Option<Coordinate> {
        match direction {
            Direction::North => self.shift_north(),
            Direction::South => self.shift_south(),
            Direction::East => self.shift_east(),
            Direction::West => self.shift_west(),
            Direction::NorthEast => self.shift_north_east(),
            Direction::NorthWest => self.shift_north_west(),
            Direction::SouthEast => self.shift_south_east(),
            Direction::SouthWest => self.shift_south_west(),
        }
    }
    pub fn shift_north(&self) -> Option<Point<isize>> {
        Some(Point {
            x: self.x,
            y: self.y.checked_sub(1)?,
        })
    }

    pub fn shift_south(&self) -> Option<Point<isize>> {
        Some(Point {
            x: self.x,
            y: self.y.checked_add(1)?,
        })
    }

    pub fn shift_east(&self) -> Option<Point<isize>> {
        Some(Point {
            x: self.x.checked_add(1)?,
            y: self.y,
        })
    }

    pub fn shift_west(&self) -> Option<Point<isize>> {
        Some(Point {
            x: self.x.checked_sub(1)?,
            y: self.y,
        })
    }
    pub fn shift_north_east(&self) -> Option<Point<isize>> {
        Some(Point {
            x: self.x.checked_add(1)?,
            y: self.y.checked_sub(1)?,
        })
    }

    pub fn shift_north_west(&self) -> Option<Point<isize>> {
        Some(Point {
            x: self.x.checked_sub(1)?,
            y: self.y.checked_sub(1)?,
        })
    }

    pub fn shift_south_east(&self) -> Option<Point<isize>> {
        Some(Point {
            x: self.x.checked_add(1)?,
            y: self.y.checked_add(1)?,
        })
    }
    pub fn shift_south_west(&self) -> Option<Point<isize>> {
        Some(Point {
            x: self.x.checked_sub(1)?,
            y: self.y.checked_add(1)?,
        })
    }

    pub fn iter_neighbours(&self) -> impl Iterator<Item=Coordinate> + '_ {
        iter_directions().into_iter().map(|d| self.shift(&d).unwrap())
    }
}


fn iter_directions() -> [Direction; 8] {
    [
        Direction::North,
        Direction::East,
        Direction::West,
        Direction::South,
        Direction::NorthWest,
        Direction::NorthEast,
        Direction::SouthEast,
        Direction::SouthWest,
    ]
}
#[derive(Clone, Debug, PartialEq, Eq, Hash, Copy)]
pub(crate) enum Direction {
    North,
    South,
    East,
    West,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
}

impl Direction {
    pub(crate) fn rotate_90_degrees_clockwise(&self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
            _ => unreachable!(),
        }
    }
    pub(crate) fn rotate_90_degrees_counter_clockwise(&self) -> Self {
        match self {
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
            Direction::North => Direction::West,
            _ => unreachable!()
        }
    }
    pub(crate) fn rotate_45_degrees_clockwise(&self) -> Self {
        match self {
            Direction::North => Direction::NorthEast,
            Direction::East => Direction::SouthEast,
            Direction::South => Direction::SouthWest,
            Direction::West => Direction::NorthWest,
            _ => unreachable!()
        }
    }
    pub(crate) fn rotate_45_degrees_counter_clockwise(&self) -> Self {
        match self {
            Direction::North => Direction::NorthWest,
            Direction::East => Direction::NorthEast,
            Direction::South => Direction::SouthEast,
            Direction::West => Direction::NorthEast,
            _ => unreachable!()
        }
    }
    pub fn get_opposite(&self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
            _ => unreachable!(),
        }
    }
}
pub(crate) trait Matrix<T> {
    fn get_scalar(&self, index: &Index) -> Option<&T>;
    fn get_mut_scalar(&mut self, index: &Index) -> Option<&mut T>;
    fn set_scalar(&mut self, index: &Index, value: T) -> Option<T>;
}

impl<T> Matrix<T> for Vec<Vec<T>> {
    fn get_scalar(&self, index: &Index) -> Option<&T> {
        self.get(index.y)?.get(index.x)
    }
    fn get_mut_scalar(&mut self, index: &Index) -> Option<&mut T> {
        self.get_mut(index.y)?.get_mut(index.x)
    }
    fn set_scalar(&mut self, index: &Index, value: T) -> Option<T> {
        let mut old_ref = self.get_mut_scalar(index)?;
        Some(mem::replace(&mut old_ref, value))
    }
}
