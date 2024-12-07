use crate::common::{Direction, Matrix, Point};
use std::iter::Extend;
type Lab = Vec<Vec<char>>;

macro_rules! unwrap_or_break {
    ($op:expr) => {
        match $op {
            Some(v) => v,
            None => break,
        }
    };
}

impl From<(isize, isize)> for Direction {
    fn from(value: (isize, isize)) -> Self {
        let signs = (value.0.signum(), value.1.signum());
        match signs {
            (-1, 0) => Direction::West,
            (1, 0) => Direction::East,
            (0, -1) => Direction::North,
            (0, 1) => Direction::South,
            _ => panic!(),
        }
    }
}
pub fn find_patrol_path(lab: &Lab) -> Option<Vec<Point>> {
    let mut patrol_path = vec![find_guard_location(lab)?];
    let mut guard_direction = Direction::North;

    loop {
        let current = patrol_path.get(patrol_path.len() - 1).unwrap();
        let next = unwrap_or_break!(current.shift(&guard_direction));
        let ch = unwrap_or_break!(lab.get_scalar(&next));
        if ch == &'.' || ch == &'^' {
            patrol_path.push(next);
        } else {
            guard_direction = match guard_direction {
                Direction::North => Direction::East,
                Direction::East => Direction::South,
                Direction::South => Direction::West,
                Direction::West => Direction::North,
                _ => unreachable!(),
            }
        }
    }
    Some(patrol_path)
}
fn find_corners(patrol_path: &Vec<Point>) -> Vec<&Point> {
   patrol_path.windows(3).filter_map(|window| {
       let (first, second, third) = (&window[0], &window[1], &window[2]);
       if first.x != third.x && first.y != third.y{
            return Some(second);
       }
       None
   }).collect()
}

pub fn find_obstruction_positions(lab: &Lab) -> Option<Vec<Point>> {
    let patrol_path = find_patrol_path(lab)?;
    let corners = find_corners(&patrol_path);
    let obstructions= corners
        .into_iter()
        .collect::<Vec<_>>()
        .windows(3)
        .filter_map(|window| {
            let (first, second, third) = (window[0], window[1], window[2]);
            let remaining = get_remaining_square_corner(first, second, third);
            // We want to take the last segment and determine its direction
            let x_diff = isize::try_from(remaining.x).unwrap() - isize::try_from(third.x).unwrap();
            let y_diff = isize::try_from(remaining.y).unwrap() - isize::try_from(third.y).unwrap();

            let direction = Direction::from((x_diff, y_diff));

            let Some(obstruction) = remaining.shift(&direction) else {
                return None
            };

            if no_points_on_segment((third, &obstruction), lab) {
                Some(obstruction)
            } else {
                None
            }
            //
        }).collect::<Vec<_>>();
    Some(obstructions)
}


fn no_points_on_segment(segment: (&Point, &Point), lab: &Lab) -> bool {
    // we know how to check the direction
    if segment.0.x == segment.1.x { // vertical
        let x = segment.0.x;
        let mut range;
        if segment.0.y > segment.1.y {
           range = segment.1.y..segment.0.y;
        } else {
            range = segment.0.y..segment.1.y;
        }
        for y in range   {
            let point = Point { x, y };
            let lab_square = lab.get_scalar(&point).unwrap();
            if lab_square == &'#' {
                return false;
            }
        }
    } else if segment.0.y == segment.0.y { // horizontal
        let y = segment.0.y;
        for x in segment.0.x..segment.1.x {
            let point = Point { x, y };
            let lab_square = lab.get_scalar(&point).unwrap();
            if lab_square == &'#' {
                return false;
            }
        }
    } else {
        panic!()
    }
    true
}
fn get_remaining_square_corner(first_corner: &Point, second_corner: &Point, third_corner: &Point) -> Point {
    if first_corner.x == second_corner.x { // vertical
        Point { x: third_corner.x, y: first_corner.y }
    } else if first_corner.y == second_corner.y { // Horizontal
        Point { x: first_corner.x, y: third_corner.y }
    } else {
        panic!()
    }
}
fn find_guard_location(lab: &Lab) -> Option<Point> {
    for y in 0..lab.len() {
        for x in 0..lab[0].len() {
            let ch = &lab[y][x];
            if ch == &'^' {
                return Some(Point { x, y });
            }
        }
    }
    None
}
#[cfg(test)]
mod tests {
    use crate::common::{Matrix, Point};
    use crate::day_06::{find_corners, find_obstruction_positions, find_patrol_path, Lab};
    use std::collections::HashSet;
    use std::hash::Hash;

    const INPUT: &str = include_str!("../resources/day_06/easy.txt");
    fn parse_input(input: &str) -> Lab {
        input
            .split("\n")
            .map(|line| line.chars().collect())
            .collect()
    }

    #[test]
    fn easy() {
        let lab = parse_input(INPUT);
        let mut left = HashSet::new();
        left.extend(find_patrol_path(&lab).unwrap());
        let left = left.len();
        let right = 4752;
        assert_eq!(left, right);
    }
    #[test]
    fn hard() {
        let lab = parse_input(INPUT);
        let mut left = HashSet::new();
        left.extend(find_obstruction_positions(&lab).unwrap());
        let left = left.len();
        let right = 4752;
        assert_eq!(left, right);
    }
    #[test]
    fn preamble() {
        let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

        let lab= parse_input(input);
        let left = find_obstruction_positions(&lab).unwrap();

        for y in 0..lab.len() {
            for x in 0..lab[0].len() {
                let point = Point { x, y };

                if left.contains(&point) {
                    print!("x");
                } else {
                    print!("{}", lab.get_scalar(&point).unwrap());
                }

            }
            print!("\n");
        }
        print!("\n\n");
        let patrol_path = find_patrol_path(&lab).unwrap();
        let corners = find_corners(&patrol_path);
        let corners:Vec<_>= corners.into_iter().map(|p|*p).collect();
        for y in 0..lab.len() {
            for x in 0..lab[0].len() {
                let point = Point { x, y };

                if corners.contains(&point) {
                    print!("x");
                } else {
                    print!("{}", lab.get_scalar(&point).unwrap());
                }

            }
            print!("\n");
        }

        print!("{}", left.len());
    }

}
