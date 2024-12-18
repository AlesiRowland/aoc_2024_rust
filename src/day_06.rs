use crate::common::{Direction, Matrix, Point};
use std::collections::HashSet;
use std::hash::Hash;
use std::iter::Extend;
use std::mem::needs_drop;

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

pub fn find_obstructions(lab: &Lab) -> Option<Vec<Point>> {
    let mut obstructions = Vec::new();
    let mut visited = Vec::new();
    let mut direction = Direction::North;
    let mut pos = find_guard_location(lab)?;
    let mut cannot_obstruct = HashSet::new();

    loop {
        let Some(next_pos) = pos.shift(&direction) else {
            return Some(obstructions);
        };

        let Some(next_tile) = lab.get_scalar(&next_pos) else {
            return Some(obstructions);
        };

        if next_tile == &'#' {
            direction = direction.rotate_90_degrees_clockwise();
            continue;
        }

        visited.push((direction.clone(), pos));
        cannot_obstruct.insert(pos);


        let obstruction = next_pos;

        if !cannot_obstruct.contains(&obstruction) {
            let mut inner_visited = Vec::new();
            let mut direction = direction.rotate_90_degrees_clockwise();
            let mut pos = pos.clone();
            loop {
                let Some(next_pos) = pos.shift(&direction) else {
                    break;
                };
                let Some(next_tile) = lab.get_scalar(&next_pos) else {
                    break;
                };

                if next_tile == &'#' || next_pos == obstruction {
                    direction = direction.rotate_90_degrees_clockwise();
                    continue;
                } else if visited.contains(&(direction.clone(), pos))
                    || inner_visited.contains(&(direction.clone(), pos))
                {
                    obstructions.push(obstruction);

                    break;
                } else {
                    inner_visited.push((direction.clone(), pos));
                    pos = next_pos;
                }
            }
        }
        pos = next_pos;
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
    use crate::day_06::{find_obstructions, find_patrol_path, Lab};
    use std::collections::HashSet;
    use std::hash::Hash;

    const INPUT: &str = include_str!("../resources/day_06/easy.txt");
    const PREAMBLE: &str = include_str!("../resources/day_06/preamble.txt");
    const DEBUG: &str = include_str!("../resources/day_06/debug.txt");

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
        left.extend(find_obstructions(&lab).unwrap());

        for y in 0..lab.len() {
            for x in 0..lab[0].len() {
                let p = Point { x, y };
                if left.contains(&p) {
                    print!("0");
                } else {
                    print!("{}", lab[y][x]);
                }
            }
            print!("\n");
        }
        assert_eq!(left.len(), 1719);
    }
}
