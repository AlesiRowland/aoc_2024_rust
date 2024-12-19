use std::collections::HashSet;
use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq, Clone, Eq, Hash, Copy)]
struct Point<T: Copy> {
    x: T,
    y: T,
}

impl Point<usize> {
    pub(crate) fn shift(
        &self,
        direction: &Direction,
    ) -> Option<Point<usize>> {
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
        }
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) enum Direction {
    North,
    South,
    East,
    West,
}
pub(crate) trait Matrix<T> {
    fn get_scalar(&self, index: &Point<usize>) -> Option<&T>;
}

impl<T> Matrix<T> for Vec<Vec<T>> {
    fn get_scalar(&self, index: &Point<usize>) -> Option<&T> {
        self.get(index.y).and_then(|row| row.get(index.x))
    }
}
pub fn find_score(terrain: &Vec<Vec<u32>>) -> usize{
    let trail_starts = get_trail_starts(terrain);
    println!("{:?}", trail_starts);
    let trail_scores = trail_starts
        .iter()
        .map(|start| find_trail_heads(start, terrain).len()).collect::<Vec<usize>>();
    println!("{:?}", trail_scores);
    trail_scores.iter().sum()

}

fn find_trail_heads(trail_start: &Point<usize>, terrain: &Vec<Vec<u32>>) -> HashSet<Point<usize>> {
    let mut trail_heads = HashSet::new();
    let mut stack = Vec::new();
    stack.push(*trail_start);

    while !stack.is_empty() {
        let mut new_stack = Vec::new();
        while let Some(current) = stack.pop() {
            let original_value = terrain.get_scalar(&current).unwrap();

            if let Some(left) = current.shift(&Direction::West) {
                if let Some(tile) = terrain.get_scalar(&left) {
                     if original_value + 1 == *tile {
                         if *tile == 9 {
                             trail_heads.insert(left);
                         } else {
                             new_stack.push(left);
                         }

                    }
                }
            };
            if let Some(right) = current.shift(&Direction::East) {
                if let Some(tile) = terrain.get_scalar(&right) {
                    if original_value + 1 == *tile {
                        if *tile == 9 {
                            trail_heads.insert(right);
                        } else {
                            new_stack.push(right);
                        }

                    }
                }
            };
            if let Some(bottom) = current.shift(&Direction::South) {
                if let Some(tile) = terrain.get_scalar(&bottom) {
                    if original_value + 1 == *tile {
                        if *tile == 9 {
                            trail_heads.insert(bottom);
                        } else {
                            new_stack.push(bottom);
                        }

                    }
                }
            }
            if let Some(top) = current.shift(&Direction::North) {
                if let Some(tile) = terrain.get_scalar(&top) {
                    if original_value + 1 == *tile {
                        if *tile == 9 {
                            trail_heads.insert(top);
                        } else {
                            new_stack.push(top);
                        }

                    }
                }
            }
        }
        stack = new_stack
    }

    trail_heads
}

fn get_trail_starts(terrain: &Vec<Vec<u32>>) -> HashSet<Point<usize>> {
    let mut trail_heads = HashSet::new();
    for row in 0..terrain.len() {
        for col in 0..terrain[row].len() {
            let index = Point { x: col, y: row };
            let tile = terrain[row][col];
            if tile == 0 {
                trail_heads.insert(index);
            }
        }
    }
    trail_heads
}

pub fn find_rating_score(terrain: &Vec<Vec<u32>>) -> usize{
    let trail_starts = get_trail_starts(terrain);
    let trail_scores = trail_starts
        .iter()
        .map(|start| get_rating(start, terrain)).collect::<Vec<usize>>();
    trail_scores.iter().sum()

}
fn get_rating(trail_start: &Point<usize>, terrain: &Vec<Vec<u32>>) -> usize {
    let mut trail_heads = Vec::new();
    let mut stack = Vec::new();
    stack.push(*trail_start);

    if trail_start == (&Point{x: 5, y: 5}) {
        println!()
    }
    while !stack.is_empty() {
        let mut new_stack = Vec::new();
        while let Some(current) = stack.pop() {
            let original_value = terrain.get_scalar(&current).unwrap();

            if let Some(left) = current.shift(&Direction::West) {
                if let Some(tile) = terrain.get_scalar(&left) {
                    if original_value + 1 == *tile {
                        if *tile == 9 {
                            trail_heads.push(left);
                        } else {
                            new_stack.push(left);
                        }

                    }
                }
            };
            if let Some(right) = current.shift(&Direction::East) {
                if let Some(tile) = terrain.get_scalar(&right) {
                    if original_value + 1 == *tile {
                        if *tile == 9 {
                            trail_heads.push(right);
                        } else {
                            new_stack.push(right);
                        }

                    }
                }
            };
            if let Some(bottom) = current.shift(&Direction::South) {
                if let Some(tile) = terrain.get_scalar(&bottom) {
                    if original_value + 1 == *tile {
                        if *tile == 9 {
                            trail_heads.push(bottom);
                        } else {
                            new_stack.push(bottom);
                        }

                    }
                }
            }
            if let Some(top) = current.shift(&Direction::North) {
                if let Some(tile) = terrain.get_scalar(&top) {
                    if original_value + 1 == *tile {
                        if *tile == 9 {
                            trail_heads.push(top);
                        } else {
                            new_stack.push(top);
                        }

                    }
                }
            }
        }
        stack = new_stack
    }

    trail_heads.len()
}
#[cfg(test)]
mod test {
    use crate::answers::{DAY_10_EASY, DAY_10_HARD};
    use crate::day_10::{find_rating_score, find_score};

    const PREAMBLE: &str = include_str!("../resources/day_10/preamble.txt");
    const EASY: &str = include_str!("../resources/day_10/easy.txt");

    fn parse_input(input: &str) -> Vec<Vec<u32>> {
        input
            .lines()
            .map(|line| line.chars().map(|ch| ch.to_digit(10).unwrap()).collect())
            .collect()
    }
    #[test]
    fn preamble() {
        let input = parse_input(PREAMBLE);
        let left = find_score(&input);
        let right = 36;
        assert_eq!(left, right);
    }
    #[test]
    fn easy() {
        let input = parse_input(EASY);
        let left = find_score(&input);
        let right = DAY_10_EASY;
        assert_eq!(left, right);
    }
    #[test]
    fn preamble_hard() {
        let input = parse_input(PREAMBLE);
        let left = find_rating_score(&input);
        let right = 81;
        assert_eq!(left, right);
    }
    #[test]
    fn hard() {
        let input = parse_input(EASY);
        let left = find_rating_score(&input);
        let right = DAY_10_HARD;
        assert_eq!(left, right);
    }
}
