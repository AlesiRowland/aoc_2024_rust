
#[derive(Clone, Debug, PartialEq, Eq, Hash, Copy)]
pub(crate) struct Point {
    pub(crate) x: usize,
    pub(crate) y: usize,
}

impl Point {
    pub(crate) fn shift(&self, direction: &Direction) -> Option<Point> {
        match direction {
            Direction::North => Some(Point {
                x: self.x,
                y: self.y.checked_sub(1)?,
            }),
            Direction::East => Some(Point {
                x: self.x.checked_add(1)?,
                y: self.y,
            }),
            Direction::South => Some(Point {
                x: self.x,
                y: self.y.checked_add(1)?,
            }),
            Direction::West => Some(Point {
                x: self.x.checked_sub(1)?,
                y: self.y,
            }),
            Direction::NorthEast => Some(Point {
                x: self.x.checked_add(1)?,
                y: self.y.checked_sub(1)?,
            }),
            Direction::NorthWest => Some(Point {
                x: self.x.checked_sub(1)?,
                y: self.y.checked_sub(1)?,
            }),
            Direction::SouthEast => Some(Point {
                x: self.x.checked_add(1)?,
                y: self.y.checked_add(1)?,
            }),
            Direction::SouthWest => Some(Point {
                x: self.x.checked_sub(1)?,
                y: self.y.checked_add(1)?,
            }),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
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
}
pub(crate) trait Matrix<T> {
    fn get_scalar(&self, point: &Point) -> Option<&T>;
}

impl<T> Matrix<T> for Vec<Vec<T>> {
    fn get_scalar(&self, point: &Point) -> Option<&T> {
        self.get(point.y)?.get(point.x)
    }
}
