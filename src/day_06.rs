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

pub fn find_patrol_path(lab: &Lab) -> Option<Vec<Point>> {
    let mut patrol_path = vec![find_guard_location(lab)?];
    let mut guard_direction = Direction::North;

    loop {
        let current = patrol_path.last().unwrap();
        let next = unwrap_or_break!(current.shift(&guard_direction));
        let ch = unwrap_or_break!(lab.get_scalar(&next));
        if ch == &'.' || ch == &'^' {
            patrol_path.push(next);
        } else {
            guard_direction = guard_direction.rotate_90_degrees_clockwise();
        }
    }
    Some(patrol_path)
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
    use crate::day_06:: {find_patrol_path, Lab};
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
    // #[test]
    // fn hard() {
    //     let lab = parse_input(INPUT);
    //     let mut left = HashSet::new();
    //     left.extend(find_obstruction_positions(&lab).unwrap());
    //     let left = left.len();
    //     let right = 4752;
    //     assert_eq!(left, right);
    // }
}
