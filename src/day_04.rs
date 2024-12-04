use std::collections::HashSet;
use std::fs::DirEntry;
use std::num::ParseIntError;

type WordSearch = Vec<Vec<char>>;

fn parse_input(input: &str) -> WordSearch {
    input.split('\n').map(|l| l.chars().collect()).collect()
}

#[derive(Clone, Debug, PartialEq)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn move_in_direction(&self, direction: &Direction) -> Option<Point> {
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

enum Direction {
    North,
    South,
    East,
    West,
}

fn iter_directions() -> [Vec<Direction>; 8] {
    [
        vec![Direction::North],
        vec![Direction::East],
        vec![Direction::South],
        vec![Direction::West],
        vec![Direction::North, Direction::East],
        vec![Direction::North, Direction::West],
        vec![Direction::South, Direction::East],
        vec![Direction::South, Direction::West],
    ]
}

fn get_word_count(word: &str, word_search: &WordSearch) -> usize {
    let word_chars = word.chars().collect::<Vec<_>>();
    let mut count = 0;
    let mut all_points = Vec::new();

    for y in (0..word_search.len()) {
        for x in (0..word_search[0].len()) {
            let start = Point { x, y };

            if let Point {x: 3, y: 9} = start {
                println!("debug");
            }

            for directions in iter_directions() {
                let word_points =
                    word_exists_in_direction(&start, &directions, &word_chars, &word_search);
                if !word_points.is_empty() {
                    count += 1;
                    all_points.extend(word_points)
                }
                // if word_exists_in_direction(&start, &directions, &word_chars, &word_search) {
                //    count += 1
                // }
            }
        }
    }
    println!("{:?}", all_points);


    for y in (0..word_search.len()) {
        for x in (0..word_search[0].len()) {
            let point = Point {x, y};
            let value;
            if all_points.contains(&point) {
                value = word_search[point.y][point.x];
            } else {
               value = '.';
            }
            print!("{}", value);
        }
        print!("\n");
    }
    count
}

fn word_exists_in_direction(
    start: &Point,
    directions: &Vec<Direction>,
    word_chars: &Vec<char>,
    word_search: &WordSearch,
) -> Vec<Point> {
    let mut captured = Vec::new();
    let mut current = start.clone();

    for char in word_chars {

        captured.push(current.clone());
        let Some(line) = word_search.get(current.y) else {
            // return false;
            return Vec::new();
        };
        let Some(ch) = line.get(current.x) else {
            // return false;
            return Vec::new();
        };
        if char != ch {
            // return false;
            return Vec::new();
        };

        for direction in directions {
            let Some(next_index) = current.move_in_direction(direction) else {
                if captured.len() == word_chars.len() {
                    return captured;
                }
                return Vec::new();
                // return false;
            };
            current = next_index;
        }
    }
    // true
    captured
}

#[cfg(test)]
mod tests {
    use crate::day_04::{get_word_count, parse_input};

    const INPUT: &str = include_str!("../resources/day_04/easy.txt");
    const PREAMBLE: &str = include_str!("../resources/day_04/preamble.txt");
    #[test]
    fn preamble() {
        let word_search = parse_input(PREAMBLE);
        let left = get_word_count("XMAS", &word_search);
        let right = 18;
        assert_eq!(left, right);
    }
    #[test]
    fn easy() {
        let word_search = parse_input(INPUT);
        let left = get_word_count("XMAS", &word_search);
        let right = 2406;
        assert_eq!(left, right);
    }
}
